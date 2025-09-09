pub mod db;
pub mod models;
pub mod schema;
pub mod structs;
pub mod error;
pub mod posts;

use actix_web::{
    web,
    App,HttpServer
};
use argon2::{Algorithm, Argon2, Params, Version};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use posts::{
    login,
    registry,
    pings
};
use db::establish_connection_pool;
use crate::posts::logout;

pub type ConnPool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection_pool().await;

    let params = match Params::new(19456, 4, 4, None){
        Ok(p) => p,
        Err(_) => return Err(
            std::io::Error::new(std::io::ErrorKind::Other, "Ошибка хэширования".to_string()))
    };

    let argon2 = Argon2::new(
        Algorithm::Argon2id, Version::V0x13, params);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(argon2.clone()))
            .app_data(web::Data::new(pool.clone()))
            .service(registry)
            .service(logout)
            .service(login)
            .service(pings)

    })
        .bind(("0.0.0.0", 5002))?
        .run()
        .await
}



