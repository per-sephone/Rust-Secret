use axum::body::Body;
use axum::extract::{Form, Path};
use axum::http::Response;
use axum::response::Redirect;
use axum::routing::{get, post};
use axum::Router;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use tera::Tera;
mod model;
use chrono::Local;
use model::Model;
use tower_http::services::ServeDir;
//use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Secret {
    pub id: i64,
    pub body: String,
    pub timestamp: String,
    pub tag: String,
    pub comments: Vec<String>,
}

#[derive(Deserialize)]
pub struct FormData {
    pub body: String,
    pub tag: String,
}

#[derive(Deserialize)]
pub struct CommentData {
    pub comment: String,
}

fn get_timestamp() -> String {
    Local::now().format("%Y-%m-%d %H:%M").to_string()
}

#[debug_handler]
async fn index() -> Result<Response<Body>, axum::body::Empty<axum::body::Bytes>> {
    let model = establish_connection();
    let entries: Vec<Secret> = model
        .select()
        .unwrap()
        .iter()
        .map(|row| Secret {
            id: row.0,
            body: row.1.clone(),
            timestamp: row.2.clone(),
            tag: row.3.clone(),
            comments: row.4.clone(),
        })
        .collect();
    //https://keats.github.io/tera/docs/
    let tera = Tera::new("templates/*.html").unwrap();
    let mut context = tera::Context::new();
    context.insert("entries", &entries);
    let rendered = tera.render("index.html", &context).unwrap();
    let response = Response::new(Body::from(rendered));
    Ok(response)
}

#[debug_handler]
async fn get_create() -> Result<Response<Body>, axum::body::Empty<axum::body::Bytes>> {
    let tera = Tera::new("templates/*.html").unwrap();
    let context = tera::Context::new();
    let rendered = tera.render("create.html", &context).unwrap();
    let response = Response::new(Body::from(rendered));
    Ok(response)
}

#[debug_handler]
async fn post_create(Form(form): Form<FormData>) -> Redirect {
    let model = establish_connection();
    let _ = model.insert(form.body, get_timestamp(), form.tag, Vec::new());
    Redirect::to("/")
}

#[debug_handler]
async fn get_comment(
    Path(id): Path<i64>,
) -> Result<Response<Body>, axum::body::Empty<axum::body::Bytes>> {
    let model = establish_connection();
    let entry = model.select_by_id(id).unwrap();
    let secret = Secret {
        id: entry.0,
        body: entry.1,
        timestamp: entry.2,
        tag: entry.3,
        comments: entry.4,
    };
    let tera = Tera::new("templates/*.html").unwrap();
    let mut context = tera::Context::new();
    context.insert("secret", &secret);
    let rendered = tera.render("comment.html", &context).unwrap();
    let response = Response::new(Body::from(rendered));
    Ok(response)
}

#[debug_handler]
async fn post_comment(Path(id): Path<i64>, Form(form): Form<CommentData>) -> Redirect {
    let model = establish_connection();
    let _ = model.add_comment(id, form.comment);
    Redirect::to(&format!("/comment/{}", id))
}

fn establish_connection() -> Model {
    Model::new().unwrap()
}

//https://docs.rs/axum/latest/axum/

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/create", get(get_create).post(post_create))
        .route("/comment/:id", get(get_comment))
        .route("/comment/comment/:id", post(post_comment))
        // https://www.joeymckenzie.tech/blog/templates-with-rust-axum-htmx-askama
        .nest_service(
            "/static",
            ServeDir::new(format!(
                "{}/static",
                std::env::current_dir().unwrap().to_str().unwrap()
            )),
        );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
