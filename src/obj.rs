pub trait Obj {
    fn get_kaiper_type(&self) -> &KType;
}

pub struct KType {
    pub name: &'static str,
}

const K_INT_TYPE: KType = KType { name: "Int" };
impl Obj for i32 {
    fn get_kaiper_type(&self) -> &KType {
        &K_INT_TYPE
    }
}

const K_NUM_TYPE: KType = KType { name: "Number" };
impl Obj for f64 {
    fn get_kaiper_type(&self) -> &KType {
        &K_NUM_TYPE
    }
}
