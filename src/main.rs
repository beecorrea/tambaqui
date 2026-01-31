mod ledger;

use ledger::database;

fn main() {
    let db = database::Database::new("my-database.duckdb");
    let l = ledger::Ledger::new(db);
    println!("Using database {}", l.database.name);
}
