pub mod database;

use crate::database::Database;
use saphyr::{LoadableYamlNode, Yaml};

#[cfg(test)]
pub mod unit;

pub struct Ledger {
    pub database: Database,
}

impl Ledger {
    pub fn new(ledger: &str) -> Self {
        let y = Yaml::load_from_str(&ledger).unwrap();
        let db = &y[0];
        let db_name = db["database"]["name"].as_str().unwrap();
        Ledger {
            database: Database::new(db_name),
        }
    }
}
