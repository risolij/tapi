use crate::models::transaction::{Transaction, PostTransaction};
use crate::errors::TransactionError;
use uuid::Uuid;
use async_trait::async_trait;
use super::repository::Repository;

pub struct TransactionRepository {
    pool: sqlx::PgPool,
}

impl TransactionRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<Transaction, PostTransaction, TransactionError> for TransactionRepository {
    async fn post(&self, other: PostTransaction) -> Result<Transaction, TransactionError> {
        let ts: Transaction = sqlx::query_as(
            "INSERT INTO transactions ( user_id, account_id, amount, created ) 
            VALUES ( $1, $2, $3, $4 ) returning *" 
        )
            .bind(other.user_id)
            .bind(other.account_id)
            .bind(other.amount)
            .bind(chrono::Utc::now())
            .fetch_one(&self.pool)
            .await
            .map_err(|_| TransactionError::TransactionInvalid)?;

        Ok(ts)
    }

    async fn get_one(&self, id: Uuid) -> Result<Option<Transaction>, TransactionError> {
        let ts: Option<Transaction> = sqlx::query_as("select * from transactions where transaction_id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| TransactionError::TransactionNotFound)?;

        Ok(ts)
    }

    async fn get_all(&self) -> Result<Vec<Transaction>, TransactionError> {
        let ts: Vec<Transaction> = sqlx::query_as("SELECT * FROM transactions")
            .fetch_all(&self.pool)
            .await
            .map_err(|_| TransactionError::TransactionNotFound)?;

        Ok(ts)
    }
}
