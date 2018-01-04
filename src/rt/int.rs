use rt::obj::Obj;
use std::rc::Rc;
use vm::VM;
use vm::err::RTErr;

macro_rules! impl_op { // TODO make another macro that reduce further redundancy
    ($id: ident, $token: tt) => {
        fn $id(&self, other: Rc<Obj>, _: &mut VM) -> Result<Rc<Obj>, RTErr> {
            if let Some(int) = other.downcast_ref::<i32>() {
                Ok(Rc::new(self $token int))
            } else if let Some (num) = other.downcast_ref::<f64>() {
                Ok(Rc::new(*self as f64 $token num))
            } else {
                Err(RTErr::TypeMismatch)
            }
        }
    };
}

impl Obj for i32 {
    impl_op!(add, +);
    impl_op!(sub, -);
    impl_op!(mul, *);
    impl_op!(div, /);
}
