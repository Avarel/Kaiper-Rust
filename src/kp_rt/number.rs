use kp_rt::obj::Obj;
use kp_rt::ktype::KType;

const K_NUM_TYPE: KType = KType { name: "Number" };
impl Obj for f64 {
    fn get_kaiper_type(&self) -> &KType {
        &K_NUM_TYPE
    }

    fn add(&self, other: &Obj) -> Box<Obj> {
        if let Some(other) = other.downcast_ref::<i32>() {
            return Box::new(self + *other as f64)
        } else if let Some(other) = other.downcast_ref::<f64>() {
            return Box::new(self + other)
        }
        unimplemented!()
    }
}