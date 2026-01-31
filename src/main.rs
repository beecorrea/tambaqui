mod ledger;

use ledger::database;

fn main() {
    let db = database::Database::new("my-database.duckdb");
    println!("{}", db.name);
}
