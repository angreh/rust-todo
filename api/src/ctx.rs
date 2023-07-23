use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Ctx {
    pub name: String,
    pub user: User,
}

impl Ctx {
    pub fn new(name: String) -> Self {
        Self {
            name,
            user: User {
                name: String::from("Angreh"),
                id: String::from("ID01"),
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
    pub name: String,
    pub id: String,
}
