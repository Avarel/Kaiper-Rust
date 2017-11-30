use ast::*;

pub trait Visitor<R, C> {
    fn visit_id(&mut self, expr: &Identifier, context: C) -> R;
}
