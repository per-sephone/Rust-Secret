//https://rust-lang-nursery.github.io/rust-cookbook/datetime.html
use chrono::Utc;

pub struct Secret {
    pub body: String,
    pub timestamp: Utc::DateTime,
    pub tags: Vec<String>,
}

pub struct Comment {
    pub body: String,
    pub timestamp: Utc::DateTime,
}