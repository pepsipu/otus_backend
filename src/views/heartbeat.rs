use crate::otus_sql::psql::{OtusDb, OtusSql};
use std::collections::HashMap;
use rocket_contrib::json::{Json, JsonValue};
use crate::serial;

#[post("/heartbeat", format="json", data="<heartbeat_data>")]
pub fn heartbeat(sql: rocket::State<HashMap<String, String>>, conn: OtusDb, heartbeat_data: Json<serial::HeartBeatData>) -> JsonValue {
    let db = OtusSql::new(&sql, &conn);
    let (competitor_id, image_id) = db.session_to_competitor(&heartbeat_data.session);
    if competitor_id == -1 {
        return json!({
            "success": false,
            "err": "bad_session"
        });
    }
    let mut score = 0;
    let mut max_score = 0;
    for vulnerability in heartbeat_data.correct.iter() {
        score += vulnerability.points;
    }
    for vulnerability in db.get_vulnerabilities(image_id) {
        max_score += vulnerability.points;
    }
    if score > max_score {
        return json!({
            "success": false,
            "err": "bad_score",
            "msg": "haxxing isn't cool :("
        });
    }
    db.create_time_slice(competitor_id, score);
    json!({
        "score": score,
        "success": true
    })
}
