use crate::utils::get_track_directory;

pub fn get_connection() -> sqlite::Connection {
    let track_dir = get_track_directory();

    let db_path = format!("{}/log.db", track_dir);

    let conn = sqlite::open(db_path).unwrap();
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

    let foods_sql = include_str!("foods.sql");

    conn.execute(foods_sql).unwrap();
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
