use crate::otus_sql::psql::{OtusDb, OtusSql};
use std::collections::HashMap;
use rocket_contrib::json::{Json, JsonValue};
use crate::serial;

#[get("/delete-image?<secret>&<image>")]
pub fn delete_image(sql: rocket::State<HashMap<String, String>>, conn: OtusDb, secret: String, image: String) -> JsonValue {
    if OtusSql::new(&sql, &conn).delete_image(image, secret) {
        return json!({
            "success": true
        });
    } else {
        return json!({
            "success": false
        });
    }

}
