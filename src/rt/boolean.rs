use rt::obj::Obj;

impl Obj for bool {
    fn truth_value(&self) -> bool {
        *self
    }
}
