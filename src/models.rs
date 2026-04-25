use uuid::Uuid;

pub struct Account {
    pub id: Uuid,
    pub site_name: String,
    pub login: Login,
    pub password: String,
}

impl Account {
    pub fn new(site_name: String, login: Login, password: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            site_name,
            login,
            password,
        }
    }
}

pub enum Login {
    Email(String),
    Username(String),
}

struct Code {
    id: Uuid,
    site_name: String,
}
