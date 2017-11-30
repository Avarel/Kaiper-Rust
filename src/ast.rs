use visitor::Visitor;

pub struct Identifier;

pub trait Expr {
    fn accept<R, C>(&self, visitor: &mut Visitor<R, C>) -> R;
}