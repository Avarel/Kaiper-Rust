use kp_rt::obj::Obj;
use kp_rt::ktype::KType;

const K_INT_TYPE: KType = KType { name: "Int" };
impl Obj for i32 {
    fn get_kaiper_type(&self) -> &KType {
        &K_INT_TYPE
    }

    fn add(&self, other: &Obj) -> Box<Obj> {
        if let Some(other) = other.downcast_ref::<i32>() {
            return Box::new(self + other)
        } else if let Some(other) = other.downcast_ref::<f64>() {
            return Box::new((*self as f64) + other)
        }
        unimplemented!()
    }
}