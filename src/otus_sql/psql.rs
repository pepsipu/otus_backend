use std::env;
use std::fs::{read_dir, read_to_string};
use std::io::{Result, Error};
use std::collections::HashMap;
use std::borrow::Borrow;
use serde::de::Unexpected::Str;
use rand;
use crate::serial::*;

#[database("otus_db")]
pub struct OtusDb(postgres::Connection);

pub struct OtusSql<'sql, 'conn> {
    sql: &'sql HashMap<String, String>,
    conn: &'conn OtusDb,
}

impl<'sql, 'conn> OtusSql<'sql, 'conn> {
    pub fn stmts() -> Result<HashMap<String, String>> {
        println!("Connected to database.");
        let mut sql_statements: HashMap<String, String> = HashMap::new();
        for entry in read_dir("./src/otus_sql/sql")? {
            let file = entry?;
            if file.file_type()?.is_file() {
                let file_name_unsplit = file.file_name().into_string().unwrap();
                let file_name: Vec<&str> = file_name_unsplit.split(".").collect();
                sql_statements.insert(String::from(file_name[0]), read_to_string(file.path())?);
            }
        }
        Ok(sql_statements)
    }
    pub fn new(sql: &'sql HashMap<String, String>, conn: &'conn OtusDb) -> OtusSql<'sql, 'conn> {
        OtusSql {
            sql,
            conn,
        }
    }
    fn create_image_container(&self, name: &String, download: &String, description: &String, secret: &String) {
        self.conn.execute(&(self.sql["create_image"]), &[name, download, description, secret]).unwrap();
    }
    pub fn create_competitor(&self, name: &String, image: i32) -> Vec<u8> {
        let random_bytes: Vec<u8> = (0..64).map(|_| { rand::random::<u8>() }).collect();
        self.conn.execute(&(self.sql["create_competitor"]), &[&image, name, &random_bytes]).unwrap();
        random_bytes
    }
    pub fn image_name_to_id(&self, name: &String) -> i32 {
        let mut id: i32 = -1;
        for row in &self.conn.query(&(self.sql["image_name_to_id"]), &[name]).unwrap() {
            id = row.get(0);
            break;
        }
        id
    }
    pub fn session_to_competitor(&self, session: &Vec<u8>) -> (i32, i32) {
        let mut id: i32 = -1;
        let mut image_id: i32 = -1;
        for row in &self.conn.query(&(self.sql["session_to_competitor"]), &[session]).unwrap() {
            id = row.get(0);
            image_id = row.get(1);
            break;
        }
        (id, image_id)
    }
    pub fn create_image(&self, image: &NewImageData) {
        self.create_image_container(&image.name, &image.download, &image.description, &image.secret);
        let image_id: i32 = self.image_name_to_id(&image.name);
        for vulnerability in &image.vulnerabilities {
            self.conn.execute(&(self.sql["create_vulnerability"]), &[&image_id, &vulnerability.command, &vulnerability.status_code, &vulnerability.success_message, &vulnerability.points]).unwrap();
        }
    }
    pub fn create_time_slice(&self, competitor: i32, score: i32) {
        self.conn.execute(&(self.sql["create_time_slice"]), &[&competitor, &score]).unwrap();
    }
    pub fn get_vulnerabilities(&self, image: i32) -> Vec<crate::serial::Vulnerability> {
        let mut vulnerabilities: Vec<crate::serial::Vulnerability> = Vec::new();
        for row in &self.conn.query(&self.sql["get_vulnerabilities"], &[&image]).unwrap() {
            vulnerabilities.push(crate::serial::Vulnerability {
                command: row.get(0),
                status_code: row.get(1),
                success_message: row.get(2),
                points: row.get(3)
            });
        }
        vulnerabilities
    }
    pub fn get_time_slices(&self, competitor_id: i32) -> Vec<crate::serial::TimeSlice> {
        let mut time_slices: Vec<crate::serial::TimeSlice> = Vec::new();
        for row in &self.conn.query(&self.sql["get_time_slices"], &[&competitor_id]).unwrap() {
            time_slices.push(crate::serial::TimeSlice {
                timestamp: row.get(0),
                current_points: row.get(1)
            })
        }
        time_slices
    }
    pub fn get_images(&self) -> Vec<Image>{
        let mut images: Vec<Image> = Vec::new();
        for row in &self.conn.query(&self.sql["get_images"], &[]).unwrap() {
            images.push(Image {
                name: row.get(0),
                download: row.get(1),
                description: row.get(2)
            });
        }
        images
    }
    pub fn delete_image(&self, image_name: String, secret: String) -> bool {
        let image_id = self.image_name_to_id(&image_name);
        let mut real_secret: String = String::from("");
        for row in &self.conn.query(&self.sql["get_secret"], &[&image_id]).unwrap() {
            real_secret = row.get(0)
        }
        if real_secret == secret {
            for line in self.sql["delete_image"].split("\n") {
                self.conn.execute(line, &[&image_id]).unwrap();
            }
            return true;
        }
        return false;
    }
    pub fn get_competitors(&self, image: i32) -> Vec<crate::serial::Competitor>  {
        let mut competitors: Vec<crate::serial::Competitor> = Vec::new();
        for row in &self.conn.query(&self.sql["get_competitors"], &[&image]).unwrap() {
            competitors.push(crate::serial::Competitor {
                id: row.get(0),
                name: row.get(1),
                score: row.get(2)
            });
        }
        return competitors;
    }
    pub fn competitor_to_image(&self, competitor: i32) -> i32 {
        let mut id: i32 = -1;
        for row in &self.conn.query(&(self.sql["competitor_image"]), &[&competitor]).unwrap() {
            id = row.get(0);
            break;
        }
        id
    }
    pub fn update_score(&self, id: i32, score: i32) {
        self.conn.execute(&self.sql["update_score.sql"], &[&id, &score]);
    }
    pub fn get_updates(&self) -> Vec<crate::serial::Update> {
        let mut updates: Vec<crate::serial::Update> = Vec::new();
        for row in &self.conn.query(&(self.sql["get_updates"]), &[]).unwrap() {
            updates.push(crate::serial::Update {
                header: row.get(0),
                content: row.get(1),
                timestamp: row.get(2)
            })
        }
        updates
    }
}
