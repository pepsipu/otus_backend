use crate::otus_sql::psql::{OtusDb, OtusSql};
use std::collections::HashMap;
use rocket_contrib::json::{Json, JsonValue};
use crate::serial;

#[get("/")]
pub fn home(sql: rocket::State<HashMap<String, String>>, conn: OtusDb) -> JsonValue {
    json!({
        "online": true
    })
}
