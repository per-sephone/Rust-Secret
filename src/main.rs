use axum::routing::{get, post};
use axum::Router;
//use std::sync::Arc;
use tera::Tera;
mod model;
use model::Model;

//https://keats.github.io/tera/docs/
fn template_init() -> Tera {
    let tera = Tera::new("templates/*.html").unwrap();
    //let tera = Arc::new(tera);

    let context = tera::Context::new();
    //context.insert("secret.body", &"test");
    //context.insert("secret.timestamp", &Utc::now().to_string());
    tera.render("index.html", &context).unwrap();
    tera.render("create.html", &context).unwrap();
    tera.render("comment.html", &context).unwrap();
    tera
}

fn database_init() -> Result<Model, rusqlite::Error> {
    //connect to the model
    let model = Model::new()?;
    Ok(model)
}

async fn index() {}

async fn create() {
    
}

async fn comment() {}

//https://docs.rs/axum/latest/axum/

#[tokio::main]
async fn main() {
    let model = database_init();
    let tera = template_init();

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
