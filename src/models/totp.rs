use rusqlite::Connection;
use totp_rs::{Algorithm, Secret, TOTP};
use uuid::Uuid;

pub const TOTP_SQL: &str = "
CREATE TABLE IF NOT EXISTS totp (
    id TEXT PRIMARY KEY,
    site_name TEXT NOT NULL,
    login TEXT NOT NULL,
    secret BLOB NOT NULL,
    UNIQUE (site_name, login)
)
";

pub struct TotpKey {
    id: Uuid,
    site_name: String,
    login: String,
    data: TOTP,
}

#[derive(Debug)]
pub enum TotpValidationError {
    EmptySiteName,
    EmptyLogin,
    BadSecret,
}

#[derive(Debug)]
pub struct TotpSaveError;

#[derive(Debug)]
pub struct TotpDeleteError;

impl TotpKey {
    pub fn new(
        site_name: String,
        login: String,
        secret: String,
    ) -> Result<Self, TotpValidationError> {
        Self::validate_site_name(&site_name)?;
        Self::validate_login(&login)?;
        let secret = Self::validate_secret(secret)?;

        Ok(Self {
            id: Uuid::new_v4(),
            site_name,
            login,
            data: TOTP::new(Algorithm::SHA1, 6, 1, 30, secret).unwrap(),
        })
    }

    pub fn get_all(conn: &Connection) -> Vec<Self> {
        let mut stmt = conn.prepare("SELECT * FROM totp").unwrap();
        stmt.query_map([], |row| {
            let id: String = row.get(0).unwrap();
            Ok(Self {
                id: Uuid::try_parse(&id).unwrap(),
                site_name: row.get(1).unwrap(),
                login: row.get(2).unwrap(),
                data: TOTP::new(
                    totp_rs::Algorithm::SHA1,
                    6,
                    1,
                    30,
                    Secret::Raw(row.get(3).unwrap()).to_bytes().unwrap(),
                )
                .unwrap(),
            })
        })
        .unwrap()
        .map(|el| el.unwrap())
        .collect()
    }

    pub fn save(&self, conn: &Connection) -> Result<(), TotpSaveError> {
        match conn.execute(
            "INSERT INTO totp
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(id)
            DO UPDATE SET
              site_name = excluded.site_name,
              login = excluded.login,
              secret = excluded.secret",
            (
                self.id.to_string(),
                &self.site_name,
                &self.login,
                &self.data.secret,
            ),
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err(TotpSaveError),
        }
    }

    pub fn delete(&self, conn: &Connection) -> Result<(), TotpDeleteError> {
        match conn.execute("DELETE FROM totp WHERE id = ?1", (self.id.to_string(),)) {
            Ok(del_count) => {
                if del_count == 1 {
                    Ok(())
                } else {
                    Err(TotpDeleteError)
                }
            }
            Err(_) => Err(TotpDeleteError),
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

    #[allow(unused)]
    pub fn secret(&self) -> &[u8] {
        &self.data.secret
    }

    pub fn gen_key(&self) -> String {
        self.data.generate_current().unwrap()
    }

    #[allow(unused)]
    pub fn set_site_name(&mut self, site_name: String) -> Result<(), TotpValidationError> {
        Self::validate_site_name(&site_name)?;
        self.site_name = site_name;
        Ok(())
    }

    #[allow(unused)]
    pub fn set_login(&mut self, login: String) -> Result<(), TotpValidationError> {
        Self::validate_login(&login)?;
        self.login = login;
        Ok(())
    }

    #[allow(unused)]
    pub fn set_secret(&mut self, secret: String) -> Result<(), TotpValidationError> {
        let sec = Self::validate_secret(secret)?;
        self.data.secret = sec;
        Ok(())
    }

    fn validate_site_name(site_name: &str) -> Result<(), TotpValidationError> {
        if !site_name.is_empty() {
            Ok(())
        } else {
            Err(TotpValidationError::EmptySiteName)
        }
    }

    fn validate_login(login: &str) -> Result<(), TotpValidationError> {
        if !login.is_empty() {
            Ok(())
        } else {
            Err(TotpValidationError::EmptyLogin)
        }
    }

    fn validate_secret(secret: String) -> Result<Vec<u8>, TotpValidationError> {
        match Secret::Encoded(secret).to_bytes() {
            Ok(secret) => Ok(secret),
            Err(_) => Err(TotpValidationError::BadSecret),
        }
    }
}
