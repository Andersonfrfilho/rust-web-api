use crate::modules::{error::constant::INVALID_ID_CODE, users::structs::User};

pub fn execute(id: &String) -> Result<User, (i32, &'static str, u16)> {
    let obj = User::origin();
    println!("{}", id);
    if id.is_empty() {
        return Err(INVALID_ID_CODE);
    }
    return Ok(obj);
}
