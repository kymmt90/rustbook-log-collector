use std::env;

use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use diesel::Connection;
use diesel::SqliteConnection;
use dotenv::dotenv;

mod handlers;

#[macro_use]
extern crate diesel;

mod db;
mod model;
mod schema;

#[derive(Clone)]
pub struct Server {
    database_url: String,
}

impl Server {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");

        Server { database_url }
    }

    pub fn establish_database_connection(&self) -> SqliteConnection {
        SqliteConnection::establish(&self.database_url).expect("Error")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::handlers::*;

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Server::new()))
            .service(web::scope("/logs").route("", web::get().to(handle_get_logs)))
    })
    .bind(("localhost", 3000))?
    .run()
    .await
}
