use axum::routing::{get, post};
use axum::Router;
use serde::Serialize;
use std::sync::Arc;
use tera::Tera;
mod model;
use model::Model;

#[derive(Serialize)]
pub struct Secret {
    pub id: i64,
    pub body: String,
    pub timestamp: String,
    pub tag: String,
}

pub struct Comment {
    pub body: String,
    pub timestamp: String,
}

async fn index() {
    let connection = establish_connection();
    let mut stmt = connection.prepare("SELECT id, body, timestamp, tag FROM secrets").unwrap();
    let secrets: Result<Vec<Secret>, rusqlite::Error> = stmt
        .query_map([], |row| {
            Ok(Secret {
                id: row.get(0)?,
                body: row.get(1)?,
                timestamp: row.get(2)?,
                tag: row.get(3)?,
            })
        }).unwrap().collect();

    let tera = Tera::new("templates/*.html").unwrap();
    let tera = Arc::new(tera);
    let mut context = tera::Context::new();
    context.insert("secrets", &secrets.unwrap());
    tera.render("index.html", &context).expect("Error rendering template");

}

async fn create() {}

async fn comment() {}

fn establish_connection() -> rusqlite::Connection {
    Model::new()
}

//https://docs.rs/axum/latest/axum/
#[tokio::main]
async fn main() {

    //init tera templating
    //https://keats.github.io/tera/docs/
    //let tera = Tera::new("templates/*.html").unwrap();
    //let tera = Arc::new(tera);
    //let context = tera::Context::new();
    //tera.render("index.html", &context).unwrap();
    //    tera.render("create.html", &context).unwrap();
    //    tera.render("comment.html", &context).unwrap();

    let app = Router::new()
        .route("/", get(index))
        .route("/create", post(create))
        .route("/comment/{id}", post(comment));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
