use crate::otus_sql::psql::{OtusDb, OtusSql};
use std::collections::HashMap;
use rocket_contrib::json::{Json, JsonValue};
use crate::serial;

#[get("/updates")]
pub fn updates(sql: rocket::State<HashMap<String, String>>, conn: OtusDb) -> JsonValue {
    let db = OtusSql::new(&sql, &conn);
    json!({
        "updates": db.get_updates()
    })
}
