#[cfg(test)]
mod unit;

pub struct Database {
    pub name: String,
}

impl Database {
    pub fn new(name: &str) -> Database {
        let db_name = if name.contains(".duckdb") {
            String::from(name)
        } else {
            format!("{}.duckdb", name)
        };

        Database {
            name: String::from(db_name),
        }
    }
}
