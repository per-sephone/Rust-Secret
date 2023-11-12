use axum::body::Body;
use axum::extract::Form;
use axum::http::Response;
use axum::response::Redirect;
use axum::routing::{get, post};
use axum::Router;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
//use std::sync::Arc;
use tera::Tera;
mod model;
use chrono::Utc;
use model::Model;
//use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Secret {
    pub id: i64,
    pub body: String,
    pub timestamp: String,
    pub tag: String,
}

/*
impl Secret {
    fn from_row(row: &Vec<serde_json::Value>) -> Secret {
        Secret {
            id: row[0].as_i64().unwrap(),
            body: row[1].as_str().unwrap_or_default().to_string(),
            timestamp: row[2].as_str().unwrap_or_default().to_string(),
            tag: row[3].as_str().unwrap_or_default().to_string(),
        }
    }
}*/

#[derive(Deserialize)]
pub struct FormData {
    pub body: String,
    pub tag: String,
}

pub struct Comment {
    pub body: String,
    pub timestamp: String,
}

#[debug_handler]
async fn index() -> Result<Response<Body>, axum::body::Empty<axum::body::Bytes>> {
    let model = establish_connection();
    let entries: Vec<Secret> = model
        .select()
        .unwrap()
        .iter()
        .map(|row| Secret {
            id: row.0.clone(),
            body: row.1.clone(),
            timestamp: row.2.clone(),
            tag: row.3.clone(),
        })
        .collect();
    let tera = Tera::new("templates/*.html").unwrap();
    //let tera = Arc::new(tera);
    let mut context = tera::Context::new();
    context.insert("entries", &entries);
    let rendered = tera.render("index.html", &context).unwrap();
    let response = Response::new(Body::from(rendered));
    Ok(response)
}

#[debug_handler]
async fn get_create() -> Result<Response<Body>, axum::body::Empty<axum::body::Bytes>> {
    let tera = Tera::new("templates/*.html").unwrap();
    //let tera = Arc::new(tera);
    let context = tera::Context::new();
    let rendered = tera.render("create.html", &context).unwrap();
    let response = Response::new(Body::from(rendered));
    Ok(response)
}

#[debug_handler]
async fn post_create(Form(form): Form<FormData>) -> Redirect {
    let model = establish_connection();
    let _ = model.insert(form.body, Utc::now().to_rfc3339(), form.tag);
    Redirect::to("/")
}

//async fn comment() {}

fn establish_connection() -> Model {
    Model::new().unwrap()
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
        .route("/create", get(get_create).post(post_create));
    //.route("/comment/{id}", post(comment));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
