use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    name: String,
}

impl User {
    pub fn origin() -> User {
        User {
            name: String::new(),
        }
    }

    pub fn new(name: &String) -> User {
        User {
            name: name.to_string(),
        }
    }
}
