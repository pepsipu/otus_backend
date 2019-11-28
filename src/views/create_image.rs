use crate::otus_sql::psql::{OtusDb, OtusSql};
use std::collections::HashMap;
use rocket_contrib::json::{Json, JsonValue};
use crate::serial;
use reqwest;

static SECRET: &str = "your captcha here";

#[post("/create-image", format = "json", data = "<new_image_data>")]
pub fn create_image(sql: rocket::State<HashMap<String, String>>, conn: OtusDb, new_image_data: Json<serial::CreateImage>) -> JsonValue {
    let client = reqwest::blocking::Client::new();
    let req = format!("https://www.google.com/recaptcha/api/siteverify?secret={}&response={}", SECRET, new_image_data.captcha.as_str());
    let res: serial::Recaptcha = serde_json::from_str(client.post(req.as_str())
        .body("")
        .send()
        .unwrap()
        .text()
        .unwrap()
        .as_str()
    ).unwrap();
    if res.success {
        OtusSql::new(&sql, &conn).create_image(&new_image_data.image);
        json!({
            "success": true
        })
    } else {
        json!({
            "success": false
        })
    }
}
