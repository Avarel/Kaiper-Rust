use kp_rt::obj::Obj;
use kp_rt::ktype::KType;

const K_STR_TYPE: KType = KType { name: "String" };
impl Obj for String {
    fn get_kaiper_type(&self) -> &KType {
        &K_STR_TYPE
    }

    fn add(&self, other: &Obj) -> Box<Obj> {
        if let Some(other) = other.downcast_ref::<String>() {
            let mut buf = self.clone();
            buf.push_str(other);
            return Box::new(buf)
        }
        unimplemented!()
    }
}