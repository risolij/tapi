extern crate env_logger;
extern crate utoipa;

use crate::repositories::transaction_repository::TransactionRepository;
use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;
use sqlx::postgres::PgPool;

mod api;
mod errors;
mod models;
mod repositories;
use models::transaction::{PostTransaction, Transaction, UpdateTransaction};

use api::transaction::{
    get_transaction_by_id, get_transactions, post_transaction, update_transaction,
};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

async fn establish_connection_pool() -> PgPool {
    let connection = std::env::var("DATABASE_URL").expect("Failed to get DB URL");
    sqlx::PgPool::connect(&connection)
        .await
        .expect("Failed to create pool")
}

#[derive(OpenApi)]
#[openapi(
    handlers(
        api::transaction::get_transactions,
        api::transaction::post_transaction,
        api::transaction::get_transaction_by_id,
        api::transaction::update_transaction
    ),
    components(Transaction, PostTransaction, UpdateTransaction)
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    info!("Establishing Connection Pool...");
    let pool = establish_connection_pool().await;

    info!("Setting up API documentation...");
    let openapi = ApiDoc::openapi();

    info!("Server starting...");
    HttpServer::new(move || {
        let transaction_repo = TransactionRepository::new(pool.clone());
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(transaction_repo))
            .service(get_transactions)
            .service(post_transaction)
            .service(get_transaction_by_id)
            .service(update_transaction)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
