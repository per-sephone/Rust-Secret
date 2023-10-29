// Base code from https://actix.rs/docs/getting-started
//use actix_files::NamedFile;
//use actix_web::{ web, App, HttpServer, Result, Error};
use tera::Tera;
use axum::{routing::get, Router,};

/* 
//https://actix.rs/docs/static-files
async fn index() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("templates/index.html")?)
}*/


//https://keats.github.io/tera/docs/
fn template_init() {
    let tera = Tera::new("templates/*.html").unwrap();
    let context = tera::Context::new();
    //context.insert("secret.body", &"test");
    //context.insert("secret.timestamp", &Utc::now().to_string());
    tera.render("index.html", &context).unwrap();
}

async fn index() {

}

//https://docs.rs/axum/latest/axum/

#[tokio::main]
async fn main() {
    template_init();
    let app = Router::new()
    .route("/", get(index));
    //.route("/foo", get(get_foo).post(post_foo))
    //.route("/foo/bar", get(foo_bar));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


/* 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    template_init();

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            //.route("/{index.html:}", web::get().to(index))
            //.service(echo)
            //.route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}*/
