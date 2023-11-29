// Used ChatGPT for help generating rust doc comments
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

/// Represents a secret entry in the application
#[derive(Serialize, Deserialize, Debug)]
pub struct Secret {
    pub id: i64,
    pub body: String,
    pub timestamp: String,
    pub tag: String,
    pub comments: Vec<String>,
}

/// Represents form data used for creating a new entry.
#[derive(Deserialize)]
pub struct FormData {
    pub body: String,
    pub tag: String,
}

/// Represents form data used for adding a comment to an entry.
#[derive(Deserialize)]
pub struct CommentData {
    pub comment: String,
}

/// Gets the current timestamp in PST in the format "%Y-%m-%d %H:%M".
pub fn get_timestamp() -> String {
    Local::now().format("%Y-%m-%d %H:%M").to_string()
}

/// Wrapper for establishing a connection to the database by creating a new `Model` instance.
pub fn establish_connection() -> Model {
    Model::new().unwrap()
}

/// Handles requests to the root path ("/") and renders the index page.
#[debug_handler]
pub async fn index() -> Result<Response<Body>, axum::body::Empty<axum::body::Bytes>> {
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
    let tera = Tera::new("templates/*.html").unwrap();
    let mut context = tera::Context::new();
    context.insert("entries", &entries);
    let rendered = tera.render("index.html", &context).unwrap();
    let response = Response::new(Body::from(rendered));
    Ok(response)
}

/// Handles GET requests to the "/create" path, rendering the page for creating a new entry.
#[debug_handler]
pub async fn get_create() -> Result<Response<Body>, axum::body::Empty<axum::body::Bytes>> {
    let tera = Tera::new("templates/*.html").unwrap();
    let context = tera::Context::new();
    let rendered = tera.render("create.html", &context).unwrap();
    let response = Response::new(Body::from(rendered));
    Ok(response)
}

/// Handles POST requests to the "/create" path, processing the form data and redirecting to the root path.
#[debug_handler]
pub async fn post_create(Form(form): Form<FormData>) -> Redirect {
    let model = establish_connection();
    let _ = model.insert(form.body, get_timestamp(), form.tag, Vec::new());
    Redirect::to("/")
}

/// Handles GET requests to the "/comment/:id" path, rendering the selected secret and comments for viewing and adding comments to an entry.
#[debug_handler]
pub async fn get_comment(
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

/// Handles POST requests to the "/comment/comment/:id" path, processing the form data and redirecting back to the entry page.
#[debug_handler]
pub async fn post_comment(Path(id): Path<i64>, Form(form): Form<CommentData>) -> Redirect {
    let model = establish_connection();
    let _ = model.add_comment(id, form.comment);
    Redirect::to(&format!("/comment/{}", id))
}

/// The main entry point of the application, defining routes and starting the server.
#[tokio::main]
pub async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/create", get(get_create).post(post_create))
        .route("/comment/:id", get(get_comment))
        .route("/comment/comment/:id", post(post_comment))
        // https://www.joeymckenzie.tech/blog/templates-with-rust-axum-htmx-askama
        .nest_service( // serves the CSS file
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
