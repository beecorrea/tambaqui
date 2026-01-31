mod test {
    use crate::ledger;

    #[test]
    fn new() {
        let serialized = "
database:
    name: my-database
";

        let l = ledger::Ledger::new(&serialized);

        assert_eq!(l.database.name, "my-database.duckdb")
    }
}
