pub mod function;

use vm::VM;
use std::rc::Rc;
use vm::err::RTErr;

#[derive(Clone)]
pub enum Obj {
    Null,
    Int(i32),
    Number(f64),
    Boolean(bool),

    String(Rc<String>),
    InternedString(Rc<String>),  // this should point to the one in the VM string pool

    NativeFunction(Rc<function::NativeFunction>)
}

impl Obj {
    pub fn add(&self, other: &Obj, _: &mut VM) -> Result<Self, RTErr> {
        match *self {
            Obj::Int(this) => int_add(this, other),
            Obj::Number(this) => num_add(this, other),
            Obj::String(ref this) => str_add(this, other),
            Obj::InternedString(ref this) => str_add(this, other),
            _ => Err(RTErr::TypeMismatch)
        }
    }

    pub fn sub(&self, other: &Obj, _: &mut VM) -> Result<Self, RTErr> {
        match *self {
            Obj::Int(this) => int_sub(this, other),
            Obj::Number(this) => num_sub(this, other),
            _ => Err(RTErr::TypeMismatch)
        }
    }

    pub fn mul(&self, other: &Obj, _: &mut VM) -> Result<Self, RTErr> {
        match *self {
            Obj::Int(this) => int_mul(this, other),
            Obj::Number(this) => num_mul(this, other),
            _ => Err(RTErr::TypeMismatch)
        }
    }

    pub fn div(&self, other: &Obj, _: &mut VM) -> Result<Self, RTErr> {
        match *self {
            Obj::Int(this) => int_div(this, other),
            Obj::Number(this) => num_div(this, other),
            _ => Err(RTErr::TypeMismatch)
        }
    }

    pub fn invoke(&self, args: Vec<Obj>, _: &mut VM) -> Result<Self, RTErr> {
        match *self {
            Obj::NativeFunction(ref nf) => (nf.func)(args),
            _ => Err(RTErr::Unimplemented)
        }
    }

    pub fn truth_value(&self) -> bool {
        match *self {
            Obj::Boolean(this) => this,
            Obj::Null => false,
            _ => true,
        }
    }
}

use std::fmt;
impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Obj::Int(this) => this.fmt(f),
            Obj::Number(this) => this.fmt(f),
            Obj::Boolean(this) => this.fmt(f),
            Obj::String(ref this) | Obj::InternedString(ref this) => this.fmt(f),
            Obj::NativeFunction(ref this) => this.fmt(f),
            Obj::Null => write!(f, "null"),
        }
    }
}

macro_rules! int_op_impl {
    ($name: ident -> $token: tt) => {
        #[inline]
        fn $name(this: i32, other: &Obj) -> Result<Obj, RTErr> {
            match *other {
                Obj::Int(other) => Ok(Obj::Int(this $token other)),
                Obj::Number(other) => Ok(Obj::Number(this as f64 $token other)),
                _ => Err(RTErr::TypeMismatch)
            }
        }
    };
}
int_op_impl!(int_add -> +);
int_op_impl!(int_sub -> -);
int_op_impl!(int_mul -> *);
int_op_impl!(int_div -> /);

macro_rules! num_op_impl {
    ($name: ident -> $token: tt) => {
        #[inline]
        fn $name(this: f64, other: &Obj) -> Result<Obj, RTErr> {
            match *other {
                Obj::Int(other) => Ok(Obj::Number(this $token other as f64)),
                Obj::Number(other) => Ok(Obj::Number(this $token other)),
                _ => Err(RTErr::TypeMismatch)
            }
        }
    };
}
num_op_impl!(num_add -> +);
num_op_impl!(num_sub -> -);
num_op_impl!(num_mul -> *);
num_op_impl!(num_div -> /);

#[inline]
fn str_add(this: &String, other: &Obj) -> Result<Obj, RTErr> {
    let mut buf = this.to_owned();
    buf.push_str(&other.to_string());
    Ok(Obj::String(Rc::new(buf)))
}