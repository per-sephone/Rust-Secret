// Base code from https://actix.rs/docs/getting-started
use actix_files::NamedFile;
use actix_web::{ web, App, HttpServer, Result, Error};

//#[get("/")]
//https://actix.rs/docs/static-files
async fn index() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("static/index.html")?)
}

/*
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/{index.html:}", web::get().to(index))
            //.service(echo)
            //.route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}