use downcast_rs::Downcast;
use std::rc::Rc;
use vm::VM;
use vm::err::RTErr;

macro_rules! stub_op {
    ($id: ident) => (stub_op!($id, Rc<Obj>, &mut VM););
    ($id: ident, $($t: ty),+) => {
        fn $id(&self, $(_: $t),+) -> Result<Rc<Obj>, RTErr> {
            Err(RTErr::Unimplemented)
        }
    };
}

use std::fmt::Display;
pub trait Obj: Display + Downcast {
    stub_op!(add);
    stub_op!(sub);
    stub_op!(mul);
    stub_op!(div);
    stub_op!(shl);
    stub_op!(shr);
    stub_op!(invoke, Vec<Rc<Obj>>, &mut VM);

    fn truth_value(&self) -> bool {
        true
    }
}

impl_downcast!(Obj);
