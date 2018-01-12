pub mod inst_writer;

use parser::ast::Expr;
use vm::compiler::inst_writer::InstWriter;
use std::collections::{HashSet, HashMap};
use byteorder::{WriteBytesExt, LE};

pub struct Compiler {
    writer: InstWriter,
    sp: Vec<String>,
    heap_scope: HashSet<String>,
    local_scope: LocalScope,
}

/*
11231
hello there lol
something completely different
monkaMEGA
something completely different
Ans: null
*/

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            writer: InstWriter::new(),
            sp: Vec::new(),
            heap_scope: HashSet::new(),
            local_scope: LocalScope::new(),
        }
    }

    fn string(&mut self, string: &String) -> usize {
        let index = self.sp.iter().position(|s| s == string);
        match index {
            Some(i) => i,
            None => {
                self.sp.push(string.to_owned());
                self.sp.len() - 1
            }
        }
    }

    /// Ok(bytes written)
    fn write(
        &mut self,
        expr: &Expr,
        used: bool, // is it used as an expr (if not pop it immediately after evaluation or never write it)
        local: bool, // affects the let statements
    ) -> Result<(), String> {
        match *expr {
            Expr::Null => if used {
                self.writer.load_null();
            },
            Expr::Int(i) => if used {
                self.writer.load_int(i);
            },
            Expr::Number(n) => if used {
                self.writer.load_num(n);
            },
            Expr::Boolean(b) => if used {
                if b {
                    self.writer.load_true();
                } else {
                    self.writer.load_false();
                }
            },
            Expr::String(ref s) => if used {
                let sp_index = self.string(s) as u16;
                self.writer.load_str(sp_index);
            },
            Expr::ExternIdent(ref id) => {
                self.heap_scope.insert(id.clone());
            }
            Expr::Ident(ref id) => if used {
                if local {
                    if let Some(local_index) = self.local_scope.index(id) {
                        self.writer.get_local(local_index);
                        return Ok(())
                    }
                }

                if !self.heap_scope.contains(id) {
                    return Err(format!("variable {} does not exist in scope", id));
                }

                let sp_index = self.string(&id) as u16;

                self.writer.get_heap(sp_index);
            },
            Expr::Let(ref name, ref expr) => {
                if used {
                    return Err(String::from("let can not be used in expr format"));
                }
                self.write(expr, true, false)?;

                if local {
                    let local_index = self.local_scope.declare(name);
                    self.writer.store_local(local_index);
                    return Ok(())
                }

                // GLOBAL
                if self.heap_scope.contains(name) {
                    return Err(format!("duplicate definition for {}", name));
                }

                self.heap_scope.insert(name.clone());
                let sp_index = self.string(&name) as u16;

                self.writer.store_heap(sp_index);

            }
            Expr::Assign(ref name, ref expr) => {
                if used {
                    return Err(String::from("assign can not be used in expr format"));
                }
                self.write(expr, true, false)?;

                if local {
                    if let Some(local_index) = self.local_scope.index(name) {
                        self.writer.store_local(local_index);
                        return Ok(())
                    }
                }

                if !self.heap_scope.contains(name) {
                    return Err(format!("variable {} does not exist in scope", name));
                }

                let sp_index = self.string(&name) as u16;
                self.writer.store_heap(sp_index);
            }
            Expr::Stmts(ref vec) => {
                for expr in &vec[0..vec.len() - 1] {
                    self.write(expr, false, local)?;
                }
                self.write(&vec[vec.len() - 1], used, local)?;
            }
            Expr::Return(ref expr) => {
                self.write(expr, true, false)?;
                self.writer.ret();
            }
            Expr::Yield(ref expr) => {
                self.write(expr, true, false)?;
                self.writer.yld();
            }
            Expr::Block(ref expr) => {
                self.local_scope.push_table();
                self.write(expr, false, true)?;
                self.local_scope.pop_table(); //todo take precautions to prevent leaking
            }
            Expr::Invoke(ref expr, ref vec) => {
                self.write(expr, true, local)?;
                for expr in vec {
                    self.write(expr, true, local)?;
                }
                self.writer.invoke(vec.len() as u8);
                if !used {
                    self.writer.pop_stack();
                }
            }
            Expr::If(ref truth, ref if_branch, ref else_branch) => {
                self.write(truth, true, false)?;

                let address = self.writer.position() + 9 /*jump_if_else*/ + Self::byte_size(if_branch, used);

                if let &Some(ref else_branch) = else_branch {
                    self.writer.jump_if_false(address + 9); // size of if_branch + jump
                    self.write(if_branch, used, local)?;

                    let address = self.writer.position() + Self::byte_size(else_branch, used) + 9/*jump*/; // size of else_branch + jump
                    self.writer.jump(address);

                    self.write(else_branch, used, local)?;
                } else {
                    self.writer.jump_if_false(address); // size of if_branch
                    self.write(if_branch, used, local)?;
                }
            }
            _ => return Err(String::from("???")),
        }

        Ok(())
    }

    fn byte_size(expr: &Expr, used: bool) -> u64 {
        macro_rules! if_use {
            ($expr: expr) => {
                if used { $expr } else { 0 }
            };
        }

        match *expr {
            // u8 (1)
            Expr::ExternIdent(_) => 0,
            Expr::Null => if_use! { 1 },
            // u8 (1) + i32 (4)
            Expr::Int(_) => if_use! { 5 },
            // u8 (1) + f64 (8)
            Expr::Number(_) => if_use! { 9 },
            // u8 (1) # both variant map into their own opcode, so only 1 byte is used
            Expr::Boolean(_) => if_use! { 1 },
            // u8 (1) + u16 (2)
            Expr::String(_) | Expr::Ident(_) => if_use! { 3 },
            // u8 (1) + u16 (2) + u16 (2) + %size of expr%
            Expr::Let(_, ref expr) | Expr::Assign(_, ref expr) => 5 + Self::byte_size(expr, true),
            // %bytes of expr...%
            Expr::Stmts(ref vec) => {
                let mut bytes = 0;
                for expr in &vec[0..vec.len() - 1] {
                    bytes += Self::byte_size(expr, false);
                }
                bytes + Self::byte_size(&vec[vec.len() - 1], used);
                bytes
            }
            // u8 (1) + %size of expr%
            Expr::Return(ref expr) | Expr::Yield(ref expr) => 1 + Self::byte_size(expr, true),
            // %size of expr%
            Expr::Block(ref expr) => Self::byte_size(expr, false),
            // u8 (1) + u8 (1) + %size of expr%
            Expr::Invoke(ref expr, ref vec) => {
                let mut bytes = 2;
                bytes += Self::byte_size(expr, true);
                for expr in vec {
                    bytes += Self::byte_size(expr, true);
                }
                bytes
            }
            // u8 (1) + u64 (8) + %size of if branch (if else + 1)% + %size of else branch% 
            Expr::If(ref truth, ref if_branch, ref else_branch) => {
                let mut bytes = 9;

                bytes += Self::byte_size(truth, true);
                bytes += Self::byte_size(if_branch, used);

                if let &Some(ref else_branch) = else_branch {
                    bytes += 9;
                    bytes += Self::byte_size(else_branch, used);
                }

                bytes
            }
            // missing binary op and unary op
            _ => unimplemented!(),
        }
    }

    pub fn compile_separate(mut self, expr: &Expr) -> Result<(Vec<u8>, Vec<String>), String> {
        self.write(expr, true, false)?;
        Ok((self.writer.complete(), self.sp))
    }

    pub fn compile(self, expr: &Expr) -> Result<Vec<u8>, String> {
        let (mut code, string_pool) = self.compile_separate(expr)?;

        let mut vec = Self::string_pool_to_vec(string_pool)?;
        vec.append(&mut code);
        Ok(vec)
    }

    fn string_pool_to_vec(string_pool: Vec<String>) -> Result<Vec<u8>, String> {
        let mut vec = Vec::new();
        vec.write_u64::<LE>(string_pool.len() as u64)
            .map_err(|_| String::from("IO error"))?;

        for s in string_pool {
            vec.write_u64::<LE>(s.len() as u64)
                .map_err(|_| String::from("IO error"))?;
            for b in s.as_bytes() {
                vec.write_u8(*b).map_err(|_| String::from("IO error"))?;
            }
        }

        Ok(vec)
    }
}


struct LocalScope {
    tables: Vec<Scope>,
    counter: u16,
}

struct Scope {
    reset: u16,
    map: HashMap<String, u16>,
}

impl LocalScope {
    fn new() -> Self {
        LocalScope {
            tables: Vec::new(),
            counter: 0,
        }
    }

    pub fn pop_table(&mut self) {
        let reset = self.tables.pop().unwrap().reset;
        self.counter = reset;
    }

    pub fn push_table(&mut self) {
        self.tables.push(Scope { reset: self.counter, map: HashMap::new() });
    }

    pub fn declare(&mut self, name: &String) -> u16 {
        let index = self
            .tables
            .last()
            .into_iter()
            .filter_map(|s| s.map.get(name).cloned())
            .nth(0)
            .unwrap_or_else(|| {
                let index = self.counter;
                self.counter += 1;
                index
            });

        self.tables.last_mut().unwrap().map.insert(name.clone(), index);

        index
    }

    pub fn index(&self, name: &String) -> Option<u16> {
        self.tables
            .iter()
            .rev()
            .filter_map(|s| s.map.get(name))
            .nth(0)
            .cloned()
    }
}
