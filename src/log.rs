use crate::db;
use crate::utils::parse_date_to_string;
use chrono::{Date, Utc};
use sqlite::Value;

pub fn log(food_id: u64, serving: f32) {
    let conn = db::get_connection();
    let date = Utc::today().format("%Y/%m/%d");

    conn.execute(format!(
        "
        INSERT INTO log (date, foodId, serving)
        VALUES ('{}', {}, {});
        ",
        date, food_id, serving
    ))
    .unwrap();
}

pub fn show_log(date: &Date<Utc>) {
    let conn = db::get_connection();
    let mut cursor = conn
        .prepare(
            "SELECT serving, name FROM log 
            LEFT OUTER JOIN food
            ON log.foodId = food.id
            WHERE date = ?",
        )
        .unwrap()
        .into_cursor()
        .bind(&[Value::String(parse_date_to_string(date))])
        .unwrap();

    while let Some(Ok(row)) = cursor.next() {
        let name = row.get::<f64, _>(0);
        let serving = row.get::<String, _>(1);
        println!("{} {}", serving, name);
    }
}
