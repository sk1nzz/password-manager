use totp_rs::{Secret, TOTP};
use uuid::Uuid;

use crate::models::Login;

pub const TOTP_SQL: &str = "
CREATE TABLE totp (
    id TEXT PRIMARY KEY,
    site_name TEXT NOT NULL,
    login TEXT NOT NULL,
    login_type TEXT NOT NULL,
    secret BLOB NOT NULL
)
";

pub struct TotpKey {
    pub id: Uuid,
    pub site_name: String,
    pub login: Login,
    pub data: TOTP,
}

impl TotpKey {
    pub fn new(site_name: String, login: Login, secret: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            site_name,
            login,
            data: TOTP::new(
                totp_rs::Algorithm::SHA1,
                6,
                1,
                30,
                Secret::Encoded(secret).to_bytes().unwrap(),
            )
            .unwrap(),
        }
    }
}
