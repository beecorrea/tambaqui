mod test {
    use crate::database;

    #[test]
    fn new() {
        let db = database::Database::new("my-database.duckdb");
        assert_eq!(db.name, "my-database.duckdb")
    }
}
