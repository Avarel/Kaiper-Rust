mod scope;
mod interpreter;
mod ast;
mod kp_rt;

fn main() {
    interpreter::test();
    // let mut what = <HashMap<&str, &str>>::new().apply_mut(|map| { map.insert("30", "30"); });

    // what.apply_mut(|map| { map.insert("30", "30"); });

    // println!("{}", what.len());
    // obj::test();
    // println!("{}", "123".to_owned().add(&"wer".to_owned()).downcast_ref::<String>().unwrap());
}
