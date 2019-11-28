use ::serde_derive::*;
#[derive(Serialize, Deserialize)]
pub struct RegisterData {
    pub image_name: String,
    pub username: String
}

#[derive(Serialize, Deserialize)]
pub struct CreateImage {
    pub image: NewImageData,
    pub captcha: String,
}
#[derive(Serialize, Deserialize)]
pub struct HeartBeatData {
    pub correct: Vec<Vulnerability>,
    pub session: Vec<u8>
}
#[derive(Serialize, Deserialize)]
pub struct NewImageData {
    pub secret: String,
    pub name: String,
    pub description: String,
    pub download: String,
    pub vulnerabilities: Vec<Vulnerability>
}
#[derive(Serialize, Deserialize)]
pub struct Vulnerability {
    pub command: String,
    pub status_code: i32,
    pub success_message: String,
    pub points: i32
}
#[derive(Serialize, Deserialize)]
pub struct Image {
    pub name: String,
    pub description: String,
    pub download: String
}

#[derive(Serialize, Deserialize)]
pub struct Recaptcha {
    pub success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Competitor {
    pub id: i32,
    pub name: String,
    pub score: i32
}

#[derive(Serialize, Deserialize)]
pub struct TimeSlice {
    pub current_points: i32,
    pub timestamp: chrono::DateTime<chrono::Utc>
}