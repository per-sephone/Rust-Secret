// Base code from https://actix.rs/docs/getting-started
use actix_files::NamedFile;
use actix_web::{ web, App, HttpServer, Result, Error};
use tera::Tera;
use lazy_static::lazy_static;

//https://actix.rs/docs/static-files
async fn index() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("static/index.html")?)
}

//https://keats.github.io/tera/docs/
fn html_template_init() {
    lazy_static! {
        pub static ref TEMPLATES: Tera = {
            let mut tera = match Tera::new("/templates/**/*") {
                Ok(t) => t,
                Err(e) => {
                    println!("Parsing error(s): {}", e);
                    ::std::process::exit(1);
                }
            };
            tera.autoescape_on(vec![".html", ".sql"]);
            //tera.register_filter("do_nothing", do_nothing_filter);
            tera
        };
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    html_template_init();

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