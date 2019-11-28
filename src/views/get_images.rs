use crate::otus_sql::psql::{OtusDb, OtusSql};
use std::collections::HashMap;
use rocket_contrib::json::{Json, JsonValue};
use crate::serial;
use rocket::Response;

#[get("/images")]
pub fn get_images(sql: rocket::State<HashMap<String, String>>, conn: OtusDb) -> JsonValue {
    let db = OtusSql::new(&sql, &conn);
    let images = db.get_images();
    json!({
        "images": images
    })
}
