use parser::ast::Expr;
use vm::inst_writer::InstWriter;
use std::collections::HashSet;

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

    pub fn write_to(expr: Expr, vec: Vec<u8>) -> Self {
        Compiler {
            writer: InstWriter::write_to(vec),
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
    fn compile(
        &mut self,
        expr: &Expr,
        used: bool, // is it used as an expr (if not pop it immediately after evaluation)
    ) -> Result<(), String> {
        match *expr {
            Expr::Null => {
                if used {
                    self.writer.load_null();
                }
            }
            Expr::Int(i) => {
                if used {
                    self.writer.load_int(i);
                }
            }
            Expr::Number(n) => {
                if used {
                    self.writer.load_num(n);
                }
            }
            Expr::Boolean(b) => {
                if used {
                    if b {
                        self.writer.load_true();
                    } else {
                        self.writer.load_false();
                    }
                }
            }
            Expr::String(ref s) => {
                if used {
                    let string_pool_index = self.string(s) as u64;
                    self.writer.load_str(string_pool_index);
                }
            }
            Expr::Identifier(ref id) => {
                if used {
                    let string_pool_index = self.string(id) as u64;
                    self.writer.get(string_pool_index);
                }
            }
            Expr::Let(ref id, ref expr) => {
                if used {
                    return Err(String::from("let can not be used in expr format"));
                }
                self.compile(expr, true)?;
                let string_pool_index = self.string(id) as u64;
                self.writer.store(0, string_pool_index);

                self.var_tables.declare(id.to_owned());
            }
            Expr::Assign(ref id, ref expr) => {
                if used {
                    return Err(String::from("assign can not be used in expr format"));
                }
                self.compile(expr, true)?;

                let table_id = self.var_tables
                    .table_index(id)
                    .ok_or(String::from("variable does not exist in scope"))?
                    as u64;

                let string_pool_index = self.string(id) as u64;
                self.writer.store(table_id, string_pool_index);
            }
            Expr::Stmts(ref vec) => {
                for expr in &vec[0..vec.len() - 1] {
                    self.compile(expr, false)?;
                }
                self.compile(&vec[vec.len() - 1], true)?;
            }
            Expr::Block(ref expr) => {
                self.writer.push_table();
                self.var_tables.push_table();

                self.compile(expr, false)?;

                self.var_tables.pop_table();
                self.writer.pop_table();
            }
            Expr::Invoke(ref expr, ref vec) => {
                self.compile(expr, true)?;
                for expr in vec {
                    self.compile(expr, true)?;
                }
                self.writer.invoke(vec.len() as u64);
                self.check(used);
            }
            _ => return Err(String::from("???")),
        }

        Ok(())
    }

    pub fn complete(mut self, expr: &Expr) -> Result<(Vec<u8>, Vec<String>), String> {
        self.compile(expr, false)?;
        Ok((self.writer.complete(), self.string_pool))
    }

    fn check(&mut self, used: bool) {
        if !used {
            self.writer.pop_stack();
        }
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
