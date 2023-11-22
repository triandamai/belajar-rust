use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod respositories;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(hello))
            .route("/get-user", web::get().to(routes::user::get_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Halo")
}
