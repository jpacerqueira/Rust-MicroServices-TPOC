use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn server_uniq_response() -> impl Responder {
    HttpResponse::Ok().body("Server is Unique and UP! RUST is great!!!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(server_uniq_response))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
