use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

const PORT: &str = "3000";

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(format!("0.0.0.0:{}",PORT))
    .unwrap_or_else(|_| panic!("Couldn't start the server at port {}",PORT))
    .run()
    .await
}
