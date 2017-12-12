use kp_rt::obj::Obj;


impl Obj for String {
    fn add(&self, other: &Obj) -> Result<Box<Obj>, String> {
        let mut buf = self.to_owned();
        buf.push_str(&other.to_string());
        Ok(Box::new(buf))
    }
}