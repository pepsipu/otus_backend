use crate::otus_sql::psql::{OtusDb, OtusSql};
use std::collections::HashMap;
use rocket_contrib::json::{Json, JsonValue};
use crate::serial;

#[post("/compete", format="json", data="<register_data>")]
pub fn compete(sql: rocket::State<HashMap<String, String>>, conn: OtusDb, register_data: Json<serial::RegisterData>) -> JsonValue {
    let db = OtusSql::new(&sql, &conn);
    let id = db.image_name_to_id(&register_data.image_name);
    if id == -1 {
        return json!({"success": false, "err": "image_void"});
    }
    if db.get_competitors(id).iter().any(|x| &x.name == &register_data.username) {
        return json!({"success": false, "err": "username_in_use"});
    }
    let session = db.create_competitor(&register_data.username, id);
    json!({"success": true, "vulnerabilities": db.get_vulnerabilities(id), "session": session})
}