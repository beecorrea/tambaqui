mod test {
    use crate::database;

    #[test]
    fn new() {
        let db_name = "my-database.duckdb";
        let db = database::Database::new(db_name);
        assert_eq!(db.name, "my-database.duckdb")
    }
}
