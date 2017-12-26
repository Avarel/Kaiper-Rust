use parser::ast::Expr;
use vm::inst_writer::InstWriter;

struct Compiler {
    writer: InstWriter
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            writer: InstWriter::new()
        }
    }

    pub fn write_to(vec: Vec<u8>) -> Self {
        Compiler {
            writer: InstWriter::write_to(vec)
        }
    }

    /// Ok(bytes written)
    pub fn compile_to(
        &mut self,
        expr: &Expr, 
        expr_form: bool // is it used as an expr (if not pop it immediately after evaluation)
    ) -> Result<usize, String> {
        let init_pos = self.writer.position();

        match *expr {
            Expr::Int(i) => {
                self.writer.load_int(i);
            }
            _ => return Err(String::from("???"))
        }

        if !expr_form { self.writer.pop_stack(); }

        Ok(self.writer.position() - init_pos)
    }
}