#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate chrono;
extern crate serde_derive;
extern crate postgres;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};
use rocket_contrib::json::{Json, JsonValue};
use rocket::http::Method;
use rocket::{get, routes};

mod serial;
mod otus_sql;
mod views;

use std::io::Result;
use otus_sql::psql::{OtusSql, OtusDb};


fn main() -> Result<()> {
    let sql_statements = OtusSql::stmts()?;
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors().unwrap();
    rocket::ignite()
        .mount("/", routes![
            views::compete::compete,
            views::create_image::create_image,
            views::get_images::get_images,
            views::home::home,
            views::delete_image::delete_image,
            views::get_competitors::get_competitors,
            views::heartbeat::heartbeat,
            views::stats::stats,
        ])
        .manage(sql_statements)
        .attach(OtusDb::fairing())
        .attach(cors)
        .launch();
    return Ok(());
}