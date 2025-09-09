use diesel_async::{AsyncPgConnection};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use dotenv::dotenv;


pub async fn establish_connection_pool() -> bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>> {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL")
        .expect("DATABASE_URL должен быть указан");
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&database_url);
    bb8::Pool::builder()
        .build(config)
        .await
        .expect("Ошибка при создании пула соединений")
}