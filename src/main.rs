use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[post("/index")]
async fn index_post(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn man_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey how are you?")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(index_post)
            .route("/hey", web::get().to(man_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
