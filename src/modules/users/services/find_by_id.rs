use crate::modules::{
    error::{constant::INVALID_ID_CODE, custom::CustomError},
    users::structs::User,
};

pub fn execute(id: &String) -> Result<User, CustomError> {
    let obj = User::origin();
    if id.is_empty() {
        return Err(CustomError::from(INVALID_ID_CODE));
    }
    return Ok(obj);
}
