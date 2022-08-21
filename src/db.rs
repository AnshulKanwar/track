pub fn get_connection() -> sqlite::Connection {
    let connection = sqlite::open("log.db").unwrap();
    connection
}

pub fn initialize_db() {
    let connection = get_connection();
    create_table(connection);
}

fn create_table(connection: sqlite::Connection) {
    connection
        .execute(
            "
        CREATE TABLE log (
            date INTEGER,
            name TEXT,
            calories INTEGER,
            carbohydrates REAL,
            fat REAL,
            protein REAL
            serving REAL
        );
        ",
        )
        .unwrap();
}
