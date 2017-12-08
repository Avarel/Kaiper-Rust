use kp_rt::obj::Obj;
use kp_rt::ktype::KType;

pub struct Null;
const K_NULL_TYPE: KType = KType { name: "Null" };
impl Obj for Null {
    fn get_kaiper_type(&self) -> &KType {
        &K_NULL_TYPE
    }

    fn add(&self, other: &Obj) -> Box<Obj> {
        unimplemented!()
    }
}