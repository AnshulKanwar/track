use std::fs;

pub fn get_connection() -> sqlite::Connection {
    let conn = sqlite::open("log.db").unwrap();
    conn
}

pub fn initialize_db() {
    let conn = get_connection();
    create_tables(&conn);
}

fn create_tables(conn: &sqlite::Connection) {
    create_table_food(conn);
    create_table_log(conn);
    println!("Initialzed Database");
}

fn create_table_food(conn: &sqlite::Connection) {
    conn.execute(
        "
        DROP TABLE IF EXISTS food;
        ",
    )
    .unwrap();

    conn.execute(
        "
        CREATE TABLE food (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            slug TEXT UNIQUE NOT NULL,
            calories INTEGER,
            carbohydrates REAL,
            fat REAL,
            protein REAL
        );
        ",
    )
    .unwrap();

    let foodsQuery = fs::read_to_string("foods.sql").unwrap();
    conn.execute(foodsQuery).unwrap();

}

fn create_table_log(conn: &sqlite::Connection) {
    conn.execute(
        "
        DROP TABLE IF EXISTS log;
        ",
    )
    .unwrap();

    conn.execute(
        "
        CREATE TABLE log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date DATE NOT NULL,
            foodId INTEGER NOT NULL,
            serving REAL NOT NULL,

            FOREIGN KEY (foodId)
                REFERENCES food (id)
                ON UPDATE CASCADE
                ON DELETE CASCADE
        );
        ",
    )
    .unwrap();
}
