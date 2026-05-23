use rusqlite::Connection;
use totp_rs::{Secret, TOTP};
use uuid::Uuid;

use crate::models::Login;

pub const TOTP_SQL: &str = "
CREATE TABLE IF NOT EXISTS totp (
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

    pub fn get_all(conn: &Connection) -> Vec<Self> {
        let mut stmt = conn.prepare("SELECT * FROM totp").unwrap();
        let mut res = Vec::new();
        let iter = stmt
            .query_map([], |row| {
                let login_type: String = row.get(3).unwrap();
                let id: String = row.get(0).unwrap();
                Ok(Self {
                    id: Uuid::try_parse(&id).unwrap(),
                    site_name: row.get(1).unwrap(),
                    login: match login_type.as_str() {
                        "email" => Login::Email(row.get(2).unwrap()),
                        "username" => Login::Username(row.get(2).unwrap()),
                        _ => panic!("bad login type"),
                    },
                    data: TOTP::new(
                        totp_rs::Algorithm::SHA1,
                        6,
                        1,
                        30,
                        Secret::Raw(row.get(4).unwrap()).to_bytes().unwrap(),
                    )
                    .unwrap(),
                })
            })
            .unwrap();

        for r in iter {
            res.push(r.unwrap());
        }

        res
    }

    pub fn save(&self, conn: &Connection) {
        let login_str = match &self.login {
            Login::Email(l) => l,
            Login::Username(l) => l,
        };

        let login_type = match self.login {
            Login::Email(_) => "email",
            Login::Username(_) => "username",
        };

        conn.execute(
            "INSERT INTO totp
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT(id)
            DO UPDATE SET
              site_name = excluded.site_name,
              login = excluded.login,
              login_type = excluded.login_type,
              secret = excluded.secret",
            (
                self.id.to_string(),
                &self.site_name,
                login_str,
                login_type,
                &self.data.secret,
            ),
        )
        .unwrap();
    }

    pub fn delete(conn: &Connection, id: Uuid) {
        conn.execute("DELETE FROM totp WHERE id = ?1", (id.to_string(),))
            .unwrap();
    }
}
