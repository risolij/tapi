use actix_web::{get, delete, patch, post, web, web::Json, HttpResponse};
use uuid::Uuid;

use crate::{
    errors::TransactionError,
    models::transaction::{PostTransaction, UpdateTransaction},
    repositories::{repository::Repository, transaction_repository::TransactionRepository},
};

type TransactionResponse = Result<HttpResponse, TransactionError>;
type Repo = web::Data<TransactionRepository>;

/// GET :: All transactions
#[utoipa::path(
    get, 
    path = "/api/transactions",
    responses(
        (status = 200, description = "Transactions found", body = [Transaction])
    )
)]
#[get("/api/transactions")]
pub async fn get_transactions(repository: Repo) -> TransactionResponse {
    let ts = repository.get_all().await?;
    Ok(HttpResponse::Ok().json(ts))
}

/// POST :: Create a new transaction
#[utoipa::path(
    post, 
    request_body = PostTransaction,
    path = "/api/transactions",
    responses(
        (status = 201, description = "Transaction created", body = Transaction),
    )
)]
#[post("/api/transactions")]
pub async fn post_transaction(
    repository: Repo,
    posted: Json<PostTransaction>,
) -> TransactionResponse {
    let ts = repository.post(posted.into_inner()).await?;
    Ok(HttpResponse::Created().json(ts))
}

/// GET :: Get single transaction by ID
#[utoipa::path(
    get, 
    path = "/api/transactions/{id}",
    responses(
        (status = 200, description = "Transaction Found!", body = Transaction),
        (status = 404, description = "Transaction not found")
    )

)]
#[get("/api/transactions/{id}")]
pub async fn get_transaction_by_id(repository: Repo, id: web::Path<Uuid>) -> TransactionResponse {
    match repository.get_one(id.into_inner()).await? {
        Some(transaction) => Ok(HttpResponse::Ok().json(transaction)),
        None => Ok(HttpResponse::NoContent().finish()),
    }
}

/// PATCH :: Update transaction by ID
#[utoipa::path(
    patch, 
    request_body = UpdateTransaction,
    path = "/api/transactions/{id}",
    responses(
        (status = 200, description = "Transaction Updated!", body = Transaction),
        (status = 404, description = "Transaction not found")
    )
)]
#[patch("/api/transactions/{id}")]
pub async fn update_transaction(
    respository: Repo,
    updated: Json<UpdateTransaction>,
    id: web::Path<Uuid>,
) -> TransactionResponse {
    match respository
        .update(id.into_inner(), updated.into_inner())
        .await?
    {
        Some(transaction) => Ok(HttpResponse::Ok().json(transaction)),
        None => Err(TransactionError::TransactionInvalid),
    }
}

/// DELETE :: Delete transaction by ID
#[utoipa::path(
    delete, 
    path = "/api/transactions/{id}",
    responses(
        (status = 202, description = "Delete request accepted!"),
        (status = 404, description = "Transaction not found")
    )
)]
#[delete("/api/transactions/{id}")]
pub async fn delete_transaction(repository: Repo, id: web::Path<Uuid>) -> TransactionResponse {
    repository.delete(id.into_inner()).await
}
