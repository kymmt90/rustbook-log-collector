use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;

mod handlers;

#[derive(Clone)]
pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::handlers::*;

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Server {}))
            .service(web::scope("/logs").route("/", web::get().to(handle_get_logs)))
    })
    .bind(("localhost", 3000))?
    .run()
    .await
}
