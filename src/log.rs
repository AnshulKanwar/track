use crate::db;
use crate::food::get_food_id;
use crate::utils::parse_date_to_string;
use chrono::{Date, Utc};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;
use sqlite::Value;

struct TotalFood {
    calories: i64,
    carbohydrates: f64,
    fat: f64,
    protein: f64,
}

impl Default for TotalFood {
    fn default() -> Self {
        Self {
            calories: 0,
            carbohydrates: 0.0,
            fat: 0.0,
            protein: 0.0,
        }
    }
}

pub fn log(food: &str, serving: f32) {
    let food_id = get_food_id(food).unwrap();

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
            "SELECT name, serving, calories, carbohydrates, fat, protein FROM log 
            LEFT OUTER JOIN food
            ON log.foodId = food.id
            WHERE date = ?",
        )
        .unwrap()
        .into_cursor()
        .bind(&[Value::String(parse_date_to_string(date))])
        .unwrap();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            "Food",
            "Serving",
            "Calories (Cal)",
            "Carbohydrates (g)",
            "Fat (g)",
            "Protein (g)",
        ]);

    let mut total = TotalFood {
        ..Default::default()
    };

    while let Some(Ok(row)) = cursor.next() {
        let name = row.get::<String, _>(0);
        let serving = row.get::<f64, _>(1);
        let calories = row.get::<i64, _>(2);
        let carbohydrates = row.get::<f64, _>(3);
        let fat = row.get::<f64, _>(4);
        let protein = row.get::<f64, _>(5);

        total.calories += calories;
        total.carbohydrates += carbohydrates;
        total.fat += fat;
        total.protein += protein;

        table.add_row(vec![
            name,
            serving.to_string(),
            format!("{}", calories),
            format!("{:.2}", carbohydrates),
            format!("{:.2}", fat),
            format!("{:.2}", protein),
        ]);
    }

    table.add_row(vec![
        String::from("Total"),
        String::from(""),
        format!("{}", total.calories),
        format!("{:.2}", total.carbohydrates),
        format!("{:.2}", total.fat),
        format!("{:.2}", total.protein),
    ]);
    println!("{table}")
}
