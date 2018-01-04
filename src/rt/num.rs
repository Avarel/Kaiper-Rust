use rt::obj::Obj;
use std::rc::Rc;
use vm::VM;
use vm::err::RTErr;

macro_rules! impl_op {
    ($id: ident, $token: tt) => {
        fn $id(&self, other: Rc<Obj>, _: &mut VM) -> Result<Rc<Obj>, RTErr> {
            if let Some(int) = other.downcast_ref::<i32>() {
                Ok(Rc::new(*self $token *int as f64))
            } else if let Some (num) = other.downcast_ref::<f64>() {
                Ok(Rc::new(*self $token num))
            } else {
                Err(RTErr::TypeMismatch)
            }
        }
    };
}

impl Obj for f64 {
    impl_op!(add, +);
    impl_op!(sub, -);
    impl_op!(mul, *);
    impl_op!(div, /);
}
