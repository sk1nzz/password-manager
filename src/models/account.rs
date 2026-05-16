use rusqlite::Connection;
use uuid::Uuid;

pub const ACCOUNT_SQL: &str = "
CREATE TABLE IF NOT EXISTS passwords (
    id TEXT PRIMARY KEY,
    site_name TEXT NOT NULL,
    login TEXT NOT NULL,
    login_type TEXT NOT NULL,
    password TEXT NOT NULL
)
";

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

    pub fn get_all(conn: &Connection) -> Vec<Self> {
        let mut stmt = conn.prepare("SELECT * FROM passwords").unwrap();
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
                    password: row.get(4).unwrap(),
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
            "INSERT INTO passwords
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT(id)
            DO UPDATE SET
              site_name = excluded.site_name,
              login = excluded.login,
              login_type = excluded.login_type,
              password = excluded.password",
            (
                self.id.to_string(),
                &self.site_name,
                login_str,
                login_type,
                &self.password,
            ),
        )
        .unwrap();
    }

    pub fn delete(conn: &Connection, id: Uuid) {
        conn.execute("DELETE FROM passwords WHERE id = ?1", (id.to_string(),))
            .unwrap();
    }
}

pub enum Login {
    Email(String),
    Username(String),
}
