mod test {
    use crate::database;

    #[test]
    fn new_fullname() {
        let db_name = "my-database.duckdb";
        let db = database::Database::new(db_name);
        assert_eq!(db.name, "my-database.duckdb")
    }

    #[test]
    fn new_from_alias() {
        let db_name = "my-database";
        let db = database::Database::new(db_name);
        assert_eq!(db.name, "my-database.duckdb")
    }
}
