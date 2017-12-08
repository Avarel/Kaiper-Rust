use downcast_rs::Downcast;
use kp_rt::ktype::KType;

pub trait Obj: Downcast {
    fn get_kaiper_type(&self) -> &KType;
    fn add(&self, other: &Obj) -> Box<Obj>;
}
impl_downcast!(Obj);