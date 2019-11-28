use crate::otus_sql::psql::{OtusDb, OtusSql};
use std::collections::HashMap;
use rocket_contrib::json::{Json, JsonValue};

#[get("/get-competitors/<image>")]
pub fn get_competitors(sql: rocket::State<HashMap<String, String>>, conn: OtusDb, image: String) -> JsonValue {
    let db = OtusSql::new(&sql, &conn);
    json!({
        "competitors": db.get_competitors(db.image_name_to_id(&image))
    })
}
