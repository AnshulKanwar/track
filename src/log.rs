use crate::db;
use chrono::Utc;

pub fn log(food_id: u64, serving: f32) {
    let conn = db::get_connection();
    let date = Utc::now();

    conn.execute(format!(
        "
        INSERT INTO log (date, foodId, serving)
        VALUES ({}, {}, {});
        ",
        date.timestamp(),
        food_id,
        serving
    ))
    .unwrap();
}
