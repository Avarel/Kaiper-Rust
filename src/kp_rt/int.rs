use kp_rt::obj::Obj;

impl Obj for i32 {
    fn add(&self, other: &Obj) -> Result<Box<Obj>, String> {
        if let Some(int) = other.downcast_ref::<i32>() {
            Ok(Box::new(self + int))
        } else if let Some (num) = other.downcast_ref::<f64>() {
            Ok(Box::new(*self as f64 + num))
        } else {
            Err(String::from("unimplemented"))
        }
    }

    fn sub(&self, other: &Obj) -> Result<Box<Obj>, String> {
        if let Some(int) = other.downcast_ref::<i32>() {
            Ok(Box::new(self - int))
        } else if let Some (num) = other.downcast_ref::<f64>() {
            Ok(Box::new(*self as f64 - num))
        } else {
            Err(String::from("unimplemented"))
        }
    }

    fn mul(&self, other: &Obj) -> Result<Box<Obj>, String> {
        if let Some(int) = other.downcast_ref::<i32>() {
            Ok(Box::new(self * int))
        } else if let Some (num) = other.downcast_ref::<f64>() {
            Ok(Box::new(*self as f64 * num))
        } else {
            Err(String::from("unimplemented"))
        }
    }

    fn div(&self, other: &Obj) -> Result<Box<Obj>, String> {
        if let Some(int) = other.downcast_ref::<i32>() {
            Ok(Box::new(self / int))
        } else if let Some (num) = other.downcast_ref::<f64>() {
            Ok(Box::new(*self as f64 / num))
        } else {
            Err(String::from("unimplemented"))
        }
    }
}