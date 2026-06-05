use rusqlite::Connection;
use uuid::Uuid;

pub const ACCOUNT_SQL: &str = "
CREATE TABLE IF NOT EXISTS passwords (
    id TEXT PRIMARY KEY,
    site_name TEXT NOT NULL,
    login TEXT NOT NULL,
    password TEXT NOT NULL,
    UNIQUE (site_name, login)
)
";

#[derive(Debug)]
pub enum AccountValidationError {
    EmptySiteName,
    EmptyLogin,
    EmptyPassword,
}

#[derive(Debug)]
pub struct AccountSaveError;

#[derive(Debug)]
pub struct AccountDeleteError;

pub struct Account {
    id: Uuid,
    site_name: String,
    login: String,
    password: String,
}

impl Account {
    pub fn new(
        site_name: String,
        login: String,
        password: String,
    ) -> Result<Self, AccountValidationError> {
        Self::validate_site_name(&site_name)?;
        Self::validate_login(&login)?;
        Self::validate_password(&password)?;

        Ok(Self {
            id: Uuid::new_v4(),
            site_name,
            login,
            password,
        })
    }

    pub fn get_all(conn: &Connection) -> Vec<Self> {
        let mut stmt = conn.prepare("SELECT * FROM passwords").unwrap();
        stmt.query_map([], |row| {
            let id: String = row.get(0).unwrap();
            Ok(Self {
                id: Uuid::try_parse(&id).unwrap(),
                site_name: row.get(1).unwrap(),
                login: row.get(2).unwrap(),
                password: row.get(3).unwrap(),
            })
        })
        .unwrap()
        .map(|el| el.unwrap())
        .collect()
    }

    pub fn save(&self, conn: &Connection) -> Result<(), AccountSaveError> {
        match conn.execute(
            "INSERT INTO passwords
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(id)
            DO UPDATE SET
              site_name = excluded.site_name,
              login = excluded.login,
              password = excluded.password",
            (
                self.id.to_string(),
                &self.site_name,
                &self.login,
                &self.password,
            ),
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err(AccountSaveError),
        }
    }

    pub fn delete(&self, conn: &Connection) -> Result<(), AccountDeleteError> {
        match conn.execute(
            "DELETE FROM passwords WHERE id = ?1",
            (self.id.to_string(),),
        ) {
            Ok(del_count) => {
                if del_count == 1 {
                    Ok(())
                } else {
                    Err(AccountDeleteError)
                }
            }
            Err(_) => Err(AccountDeleteError),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn site_name(&self) -> &str {
        &self.site_name
    }

    pub fn login(&self) -> &str {
        &self.login
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn set_site_name(&mut self, site_name: String) -> Result<(), AccountValidationError> {
        Self::validate_site_name(&site_name)?;
        self.site_name = site_name;
        Ok(())
    }

    pub fn set_login(&mut self, login: String) -> Result<(), AccountValidationError> {
        Self::validate_login(&login)?;
        self.login = login;
        Ok(())
    }

    pub fn set_password(&mut self, password: String) -> Result<(), AccountValidationError> {
        Self::validate_password(&password)?;
        self.password = password;
        Ok(())
    }

    fn validate_site_name(site_name: &str) -> Result<(), AccountValidationError> {
        if !site_name.is_empty() {
            Ok(())
        } else {
            Err(AccountValidationError::EmptySiteName)
        }
    }

    fn validate_login(login: &str) -> Result<(), AccountValidationError> {
        if !login.is_empty() {
            Ok(())
        } else {
            Err(AccountValidationError::EmptyLogin)
        }
    }

    fn validate_password(password: &str) -> Result<(), AccountValidationError> {
        if !password.is_empty() {
            Ok(())
        } else {
            Err(AccountValidationError::EmptyPassword)
        }
    }
}
