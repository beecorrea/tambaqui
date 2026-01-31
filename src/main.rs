mod ledger;

use ledger::database;

fn main() {
    let serialized = "
database:
    name: my-database
";

    let l = ledger::Ledger::new(&serialized);
    println!("Using database {}", l.database.name);
}
