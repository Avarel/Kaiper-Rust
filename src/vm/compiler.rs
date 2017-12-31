use parser::ast::Expr;
use vm::inst_writer::InstWriter;
use std::collections::HashSet;
use byteorder::{ByteOrder, WriteBytesExt, LE};

pub struct Compiler {
    writer: InstWriter,
    string_pool: Vec<String>,
    var_tables: CompilerVarTable,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            writer: InstWriter::new(),
            string_pool: Vec::new(),
            var_tables: CompilerVarTable::new(),
        }
    }

    pub fn write_to(buf: Vec<u8>) -> Self {
        Compiler {
            writer: InstWriter::write_to(buf),
            string_pool: Vec::new(),
            var_tables: CompilerVarTable::new(),
        }
    }

    fn string(&mut self, string: &String) -> usize {
        let index = self.string_pool.iter().position(|s| s == string);
        match index {
            Some(i) => i,
            None => {
                self.string_pool.push(string.to_owned());
                self.string_pool.len() - 1
            }
        }
    }

    /// Ok(bytes written)
    fn write(
        &mut self,
        expr: &Expr,
        used: bool, // is it used as an expr (if not pop it immediately after evaluation)
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
                let string_pool_index = self.string(s) as u16;
                self.writer.load_str(string_pool_index);
            },
            Expr::Identifier(ref id) => if used {
                let string_pool_index = self.string(id) as u16;
                self.writer.get(string_pool_index);
            },
            Expr::Let(ref id, ref expr) => {
                if used {
                    return Err(String::from("let can not be used in expr format"));
                }
                self.write(expr, true)?;
                let string_pool_index = self.string(id) as u16;
                self.writer.store(0, string_pool_index);

                self.var_tables.declare(id.to_owned());
            }
            Expr::Assign(ref id, ref expr) => {
                if used {
                    return Err(String::from("assign can not be used in expr format"));
                }
                self.write(expr, true)?;

                let table_id = self.var_tables
                    .table_index(id)
                    .ok_or(String::from("variable does not exist in scope"))?
                    as u16;

                let string_pool_index = self.string(id) as u16;
                self.writer.store(table_id, string_pool_index);
            }
            Expr::Stmts(ref vec) => {
                for expr in &vec[0..vec.len() - 1] {
                    self.write(expr, false)?;
                }
                self.write(&vec[vec.len() - 1], used)?;
            }
            Expr::Return(ref expr) => {
                self.write(expr, true)?;
                self.writer.ret();
            }
            Expr::Yield(ref expr) => {
                self.write(expr, true)?;
                self.writer.yld();
            }
            Expr::Block(ref expr) => {
                self.writer.push_table();
                self.var_tables.push_table();

                self.write(expr, false)?;

                self.var_tables.pop_table();
                self.writer.pop_table();
            }
            Expr::Invoke(ref expr, ref vec) => {
                self.write(expr, true)?;
                for expr in vec {
                    self.write(expr, true)?;
                }
                self.writer.invoke(vec.len() as u8);
                if !used {
                    self.writer.pop_stack();
                }
            }
            Expr::If(ref truth, ref if_branch, ref else_branch) => {
                self.write(truth, true)?;

                let address = self.writer.position() + 9 /*jump_if_else*/ + Self::byte_size(if_branch, used);

                if let &Some(ref else_branch) = else_branch {
                    self.writer.jump_if_false(address + 9); // size of if_branch + jump
                    self.write(if_branch, used)?;

                    let address = self.writer.position() + Self::byte_size(else_branch, used) + 9/*jump*/; // size of else_branch + jump
                    self.writer.jump(address);

                    self.write(else_branch, used)?;
                } else {
                    self.writer.jump_if_false(address); // size of if_branch
                    self.write(if_branch, used)?;
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
            Expr::Null => if_use! { 1 },
            // u8 (1) + i32 (4)
            Expr::Int(_) => if_use! { 5 },
            // u8 (1) + f64 (8)
            Expr::Number(_) => if_use! { 9 },
            // u8 (1) # both variant map into their own opcode, so only 1 byte is used
            Expr::Boolean(_) => if_use! { 1 },
            // u8 (1) + u16 (2)
            Expr::String(_) | Expr::Identifier(_) => if_use! { 3 },
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
            // u8 (1) + %size of expr% + u8 (1)
            Expr::Block(ref expr) => 2 + Self::byte_size(expr, false),
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
        self.write(expr, true)?;
        Ok((self.writer.complete(), self.string_pool))
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

use std::cell::RefCell;

struct CompilerVarTable {
    tables: Vec<RefCell<HashSet<String>>>,
}

impl CompilerVarTable {
    fn new() -> Self {
        CompilerVarTable {
            tables: vec![RefCell::new(HashSet::new())],
        }
    }

    pub fn pop_table(&mut self) {
        self.tables.pop();
    }

    pub fn push_table(&mut self) {
        self.tables.push(RefCell::new(HashSet::new()));
    }

    pub fn declare(&mut self, name: String) {
        RefCell::borrow_mut(self.tables.last().unwrap()).insert(name);
    }

    pub fn table_index(&self, name: &String) -> Option<usize> {
        self.tables
            .iter()
            .rev()
            .map(|c| RefCell::borrow_mut(c))
            .position(|t| t.contains(name))
    }
}
