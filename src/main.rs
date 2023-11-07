use axum::routing::get;
use axum::Router;
use axum::http::Response;
use axum::body::Body;
use axum_macros::debug_handler;
use serde::Serialize;
//use std::sync::Arc;
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

#[debug_handler]
async fn index() -> Result<Response<Body>, axum::body::Empty<axum::body::Bytes>> {
    let model = establish_connection();
    let entries = model.select().unwrap();
    let tera = Tera::new("templates/*.html").unwrap();
    //let tera = Arc::new(t);
    let mut context = tera::Context::new();
    context.insert("entries", &entries);
    let rendered = tera.render("index.html", &context).unwrap();
    let response = Response::new(Body::from(rendered));
    Ok(response)
}

//async fn create() {}

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
        .route("/", get(index));
        //.route("/create", post(create))
        //.route("/comment/{id}", post(comment));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
