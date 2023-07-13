use crate::modules::users::structs::User;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
#[non_exhaustive]
pub struct MyError {
    name: &'static str,
}

impl MyError {
    pub fn origin() -> MyError {
        MyError { name: "" }
    }

    pub fn new(name: &'static str) -> MyError {
        MyError { name }
    }
}

pub fn execute(id: &String) -> Result<User, MyError> {
    let obj = User::origin();
    println!("{}", id);
    if id.is_empty() {
        return Err(MyError::origin());
    }
    return Ok(obj);
}
