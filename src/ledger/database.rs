#[cfg(test)]
mod unit;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub name: String,
}

impl Database {
    pub fn new(name: &str) -> Database {
        Database {
            name: String::from(name),
        }
    }
}
