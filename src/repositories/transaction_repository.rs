use super::repository::Repository;
use crate::errors::TransactionError;
use crate::models::transaction::{PostTransaction, Transaction, UpdateTransaction};
use async_trait::async_trait;
use uuid::Uuid;
use actix_web::HttpResponse;

pub struct TransactionRepository {
    pool: sqlx::PgPool,
}

impl TransactionRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<Transaction, PostTransaction, UpdateTransaction, TransactionError>
    for TransactionRepository
{
    async fn post(&self, other: PostTransaction) -> Result<Transaction, TransactionError> {
        const QUERY: &str = "
        INSERT INTO transactions 
            ( user_id, account_id, amount, created, category ) 
        VALUES 
            ( $1, $2, $3, $4, $5 ) 
        returning *";

        let ts: Transaction = sqlx::query_as(QUERY)
            .bind(other.user_id)
            .bind(other.account_id)
            .bind(other.amount)
            .bind(chrono::Utc::now())
            .bind(other.category)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| TransactionError::TransactionInvalid)?;

        Ok(ts)
    }

    async fn get_one(&self, id: Uuid) -> Result<Option<Transaction>, TransactionError> {
        const QUERY: &str = "select * from transactions where transaction_id = $1";

        let ts: Option<Transaction> = sqlx::query_as(QUERY)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| TransactionError::TransactionNotFound)?;

        Ok(ts)
    }

    async fn get_all(&self) -> Result<Vec<Transaction>, TransactionError> {
        const QUERY: &str = "SELECT * FROM transactions";

        let ts: Vec<Transaction> = sqlx::query_as(QUERY)
            .fetch_all(&self.pool)
            .await
            .map_err(|_| TransactionError::TransactionNotFound)?;

        Ok(ts)
    }

    async fn update(
        &self,
        id: Uuid,
        other: UpdateTransaction,
    ) -> Result<Option<Transaction>, TransactionError> {
        const QUERY: &str = "
            UPDATE transactions SET 
                amount = $1, 
                category = $2 
            WHERE transaction_id = $3 returning *";

        let ts: Option<Transaction> = sqlx::query_as(QUERY)
            .bind(other.amount)
            .bind(other.category)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| TransactionError::TransactionInvalid)?;

        Ok(ts)
    }

    async fn delete(&self, id: Uuid) -> Result<HttpResponse, TransactionError> {
        const QUERY: &str = "DELETE from transactions WHERE transaction_id = $1";

        let record = sqlx::query(QUERY)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|_| TransactionError::TransactionInvalid)?
            .rows_affected();

        if record > 0 {
            return Ok(HttpResponse::Accepted().finish());
        } else {
            return Err(TransactionError::TransactionNotFound);
        }

    }
}
