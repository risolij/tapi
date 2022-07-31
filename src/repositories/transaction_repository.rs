use super::repository::Repository;
use crate::errors::TransactionError;
use crate::models::transaction::{PostTransaction, Transaction, UpdateTransaction};
use async_trait::async_trait;
use uuid::Uuid;
use actix_web::HttpResponse;
use crate::lib::Schema;

pub struct TransactionRepository {
    pool: sqlx::PgPool,
}

impl TransactionRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

type TransactionResult = Result<Transaction, TransactionError>;
type MultiTransacationResult = Result<Vec<Transaction>, TransactionError>;
type OptionalTransactionResult = Result<Option<Transaction>, TransactionError>;

#[async_trait]
impl Repository<Transaction, PostTransaction, UpdateTransaction, TransactionError>
    for TransactionRepository
{
    async fn post(&self, other: PostTransaction) -> TransactionResult {
        sqlx::query_as(Transaction::sql_insert())
            .bind(other.user_id)
            .bind(other.account_id)
            .bind(other.amount)
            .bind(chrono::Utc::now())
            .bind(other.category)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| TransactionError::DatabaseError(e))
        
    }

    async fn get_one(&self, id: Uuid) -> OptionalTransactionResult {
        sqlx::query_as(Transaction::sql_select_by_id())
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| TransactionError::DatabaseError(e))
    }

    async fn get_all(&self) -> MultiTransacationResult {
        sqlx::query_as(Transaction::sql_select())
            .fetch_all(&self.pool)
            .await
            .map_err(|e| TransactionError::DatabaseError(e))
    }

    async fn update(&self, id: Uuid, other: UpdateTransaction) -> OptionalTransactionResult {

        sqlx::query_as(Transaction::sql_update())
            .bind(other.amount)
            .bind(other.category)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| TransactionError::DatabaseError(e))
    }

    async fn delete(&self, id: Uuid) -> Result<HttpResponse, TransactionError> {
        sqlx::query(Transaction::sql_delete())
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| TransactionError::DatabaseError(e))?;


        Ok(HttpResponse::Accepted().finish())
    }
}
