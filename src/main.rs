#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

mod pool;
mod errors;
mod data_access;
mod schema;
mod models;
mod handlers;
mod routes;

use warp::{Filter};
use log::{info};

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Builder};
use crate::errors::{AppError};
use crate::pool::AsyncPool;
use serde::{Serialize, Deserialize};
use std::net::Ipv4Addr;

fn pg_pool(db_url: &str) -> AsyncPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    let pool = Builder::new()
        .max_size(10)
        .build(manager).unwrap();
    
    AsyncPool::new(pool, 10)
}


diesel_migrations::embed_migrations!("./migrations");

#[derive(Serialize, Deserialize, Debug)]
struct EnvConfigIn {
    database_url: String,
    http_host: Ipv4Addr,
    http_port: u16,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let config = match envy::from_env::<EnvConfigIn>() {
        Ok(config) => config,
        Err(error) => panic!("Service started with invalid environment variables {:#?}", error)
    };

    let pg_pool = pg_pool(config.database_url.as_str());

    let result = pg_pool.get(|connection| {
        diesel_migrations::run_pending_migrations(connection)
    }).await;

    match result {
        Ok(()) => {},
        Err(err) => {
            println!("error run migrations {:?}", err);
            return;
        }
    };
    
    info!("Migrations ok");


    let customer_routes = routes::customer_routes(pg_pool)
        .recover(errors::handle_rejection);


    info!("Starting server on port 3030...");

    // Start up the server...
    warp::serve(customer_routes).run((config.http_host, config.http_port)).await;
}