use rt::obj::Obj;

macro_rules! impl_op {
    ($id: ident, $token: tt) => {
        fn $id(&self, other: &Obj) -> Result<Box<Obj>, String> {
            if let Some(int) = other.downcast_ref::<i32>() {
                Ok(Box::new(self $token *int as f64))
            } else if let Some (num) = other.downcast_ref::<f64>() {
                Ok(Box::new(self $token num))
            } else {
                Err(String::from("unimplemented"))
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