pub mod database;

use serde::{Deserialize, Serialize};

use crate::database::Database;

#[cfg(test)]
pub mod unit;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ledger {
    pub database: Database,
}

impl Ledger {
    pub fn new(database: Database) -> Self {
        Ledger { database: database }
    }
}
