#[macro_use]
extern crate diesel;

mod pool;
mod data_access;
mod schema;
mod models;
mod handlers;

use std::env;
use actix_web::{middleware, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::pool::AsyncPool;
use crate::data_access::DBAccessManager;

use dotenv::dotenv;

fn pg_pool(db_url: &str) -> AsyncPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::new(manager).expect("Postgres connection pool could not be created");

    AsyncPool::new(pool, num_cpus::get())
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL env not set");

    let pg_pool = pg_pool(database_url.as_str());

    let db_manger = DBAccessManager::new(pg_pool);

    let bind = "127.0.0.1:3000";

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(db_manger.clone())
            .wrap(middleware::Logger::default())
            .service(handlers::customers_list)
            .service(handlers::get_customer)
            .service(handlers::create_customer)
            .service(handlers::update_customer)
            .service(handlers::delete_customer)
    })
    .bind(&bind)?
    .run()
    .await
}
