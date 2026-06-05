use rusqlite::Connection;
use std::{env, fs, path::PathBuf};

use crate::models::account::ACCOUNT_SQL;
use crate::models::totp::TOTP_SQL;

const BASE_PATH: &str = ".local/share/password-manager";
const DB_FILENAME: &str = "data.db";

fn get_base_path() -> PathBuf {
    let mut path = env::home_dir().expect("Can't get the home directory path");
    path.push(BASE_PATH);
    path
}

pub fn check_db_exists() -> bool {
    let mut path = get_base_path();
    path.push(DB_FILENAME);

    fs::exists(path).expect("Can't check the database file existence")
}

pub fn init_db_with_password(password: &str) -> Connection {
    let mut path = get_base_path();
    fs::create_dir_all(&path).expect("Can't create directories for the DB");
    path.push(DB_FILENAME);

    let db = rusqlite::Connection::open(path).expect("DB connection failed");

    db.pragma_update(None, "key", password).unwrap();

    db.execute(ACCOUNT_SQL, ()).unwrap();
    db.execute(TOTP_SQL, ()).unwrap();

    db
}

pub fn unlock_db(password: &str) -> Result<Connection, ()> {
    let mut path = get_base_path();
    path.push(DB_FILENAME);

    let db = rusqlite::Connection::open(path).expect("DB connection failed");

    if let Err(_) = db.pragma_update(None, "key", password) {
        return Err(());
    }

    match db.query_row("SELECT count(*) FROM sqlite_master", (), |row| {
        row.get::<_, i32>(0)
    }) {
        Ok(_) => Ok(db),
        Err(_) => Err(()),
    }
}

pub fn validate_password(password: &str) -> Result<(), ()> {
    if password.chars().count() >= 8 {
        Ok(())
    } else {
        Err(())
    }
}
