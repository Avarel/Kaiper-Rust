#[macro_use]
extern crate downcast_rs;

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
}

