use kp_rt::obj::Obj;

// pub fn add(s: &String, other: &Obj) -> Result<Obj, String> {
//     let mut buf = s.to_owned();
//     buf.push_str(&other.to_string());
//     Ok(Obj::String(buf))
// }

impl Obj for String {

}