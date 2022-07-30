extern crate env_logger;

use actix_web::{App, HttpServer, middleware::Logger, web};
use crate::repositories::transaction_repository::TransactionRepository;
use log::info;
use sqlx::postgres::PgPool;

mod api;
mod models;
mod repositories;
mod errors;

use api::transaction::{
    post_transaction, 
    get_transactions, 
    get_transaction_by_id,
    update_transaction
};

async fn establish_connection_pool() -> PgPool {
    let connection = std::env::var("DATABASE_URL").expect("Failed to get DB URL");
    sqlx::PgPool::connect(&connection)
        .await
        .expect("Failed to create pool")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    info!("Establishing Connection Pool...");
    let pool = establish_connection_pool().await;

    info!("Server starting...");
    HttpServer::new( move || {
        let transaction_repo = TransactionRepository::new(pool.clone());
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(transaction_repo))
            .service(get_transactions)
            .service(post_transaction)
            .service(get_transaction_by_id)
            .service(update_transaction)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
