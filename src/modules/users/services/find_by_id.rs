use crate::modules::users::structs::User;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
pub struct MyError {
    name: &'static str,
}

pub fn execute(id: &String) -> Result<User, MyError> {
    let obj = User {
        name: "".to_string(),
    };
    if id.is_empty() {
        return Err(MyError { name: "sei lah" });
    }
    return Ok(obj);
}
