use crate::otus_sql::psql::{OtusDb, OtusSql};
use std::collections::HashMap;
use rocket_contrib::json::{Json, JsonValue};
use crate::serial;

#[get("/stats/<competitor_id>")]
pub fn stats(sql: rocket::State<HashMap<String, String>>, conn: OtusDb, competitor_id: i32) -> JsonValue {
    let db = OtusSql::new(&sql, &conn);
    let mut max_score = 0;
    for vulnerability in db.get_vulnerabilities(db.competitor_to_image(competitor_id)) {
        max_score += vulnerability.points;
    }
    json!({
        "max_score": max_score,
        "time_slices": db.get_time_slices(competitor_id)
    })
}
