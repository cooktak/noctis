use actix_web::{App, get, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
        .service(index)
    }).keep_alive(75);

    server.bind("127.0.0.1:8088")?.run().await
}
