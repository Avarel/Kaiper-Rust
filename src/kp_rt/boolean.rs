use kp_rt::obj::Obj;
use kp_rt::ktype::KType;

const K_BOOL_TYPE: KType = KType { name: "Boolean" };
impl Obj for bool {
    fn get_kaiper_type(&self) -> &KType {
        &K_BOOL_TYPE
    }
}