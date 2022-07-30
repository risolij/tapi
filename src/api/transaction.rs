use actix_web::{
    get,
    post,
    patch,
    web,
    web::Json,
    HttpResponse,
};
use uuid::Uuid;

use crate::{
    models::transaction::{PostTransaction, UpdateTransaction}, 
    errors::TransactionError,
    repositories::{
        repository::Repository,
        transaction_repository::TransactionRepository,
    },
};


type TransactionResponse = Result<HttpResponse, TransactionError>;
type Repo = web::Data<TransactionRepository>;


/// GET :: All transactions
#[get("/api/transactions")]
pub async fn get_transactions(repository: Repo) -> TransactionResponse {
    let ts = repository.get_all().await?;
    Ok(HttpResponse::Ok().json(ts))
}


/// POST :: Create a new transaction
#[post("/api/transactions")]
pub async fn post_transaction(
    repository: Repo,
    posted: Json<PostTransaction>
) -> TransactionResponse {
    let ts = repository.post(posted.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ts))
}


/// GET :: Get single transaction by ID
#[get("/api/transactions/{id}")]
pub async fn get_transaction_by_id(repository: Repo, id: web::Path<Uuid>) -> TransactionResponse {
    match repository.get_one(id.into_inner()).await? {
        Some(transaction) => Ok(HttpResponse::Ok().json(transaction)),
        None => Ok(HttpResponse::NoContent().finish()),
    }
}

/// PATCH :: Update transaction by ID
#[patch("/api/transactions/{id}")]
pub async fn update_transaction(
    respository: Repo, 
    updated: Json<UpdateTransaction>,
    id: web::Path<Uuid> 
) -> TransactionResponse {

    match respository.update(id.into_inner(), updated.into_inner()).await? {
        Some(transaction) => Ok(HttpResponse::Ok().json(transaction)),
        None => Err(TransactionError::TransactionInvalid),
    }
}
