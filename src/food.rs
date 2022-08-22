use crate::db;
use sqlite::Value;

// TODO: returning error
pub fn get_food_id(slug: &str) -> Result<u64, &'static str> {
    let conn = db::get_connection();

    let mut cursor = conn
        .prepare("SELECT id FROM food WHERE slug = ?")
        .unwrap()
        .into_cursor()
        .bind(&[Value::String(slug.to_string())])
        .unwrap();

    let id: i64;

    while let Some(Ok(row)) = cursor.next() {
        id = row.get::<i64, _>(0);
        return Ok(id as u64);
    }

    return Err("food not found");
}
