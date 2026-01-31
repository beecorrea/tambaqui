mod ledger;

use ledger::database;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let ledger_config = fs::read_to_string("ledger.example.yaml")?;
    let l = ledger::Ledger::new(&ledger_config);
    println!("Using database {}", l.database.name);

    Ok(())
}
