extern crate env_logger;
extern crate utoipa;
extern crate strum;
#[macro_use] 
extern crate strum_macros;

use crate::repositories::transaction_repository::TransactionRepository;
use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;

mod controllers;
mod errors;
mod models;
mod repositories;
mod lib;

use models::api_doc::ApiDoc;
use crate::lib::establish_connection_pool;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use controllers::transaction::{
    get_transactions,
    post_transaction,
    get_transaction_by_id,
    delete_transaction,
    update_transaction
};


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
            .service(delete_transaction)
            .service(update_transaction)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-doc/openapi.json", openapi.clone())
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
