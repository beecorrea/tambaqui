mod test {
    use crate::database;
    use crate::ledger;

    #[test]
    fn new() {
        let db_name = "my-database.duckdb";
        let db = database::Database::new(db_name);

        let l = ledger::Ledger::new(db);
        assert_eq!(l.database.name, db_name)
    }
}
