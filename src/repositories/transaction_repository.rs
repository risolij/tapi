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

type TransactionResult = Result<Transaction, TransactionError>;
type MultiTransacationResult = Result<Vec<Transaction>, TransactionError>;
type OptionalTransactionResult = Result<Option<Transaction>, TransactionError>;

#[async_trait]
impl Repository<Transaction, PostTransaction, UpdateTransaction, TransactionError>
    for TransactionRepository
{
    async fn post(&self, other: PostTransaction) -> TransactionResult {
        const QUERY: &str = "
        INSERT INTO transactions 
            ( user_id, account_id, amount, created, category ) 
        VALUES 
            ( $1, $2, $3, $4, $5 ) 
        returning *";

        sqlx::query_as(QUERY)
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
        const QUERY: &str = "select * from transactions where transaction_id = $1";

        sqlx::query_as(QUERY)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| TransactionError::DatabaseError(e))
    }

    async fn get_all(&self) -> MultiTransacationResult {
        const QUERY: &str = "SELECT * FROM transactions";

        sqlx::query_as(QUERY)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| TransactionError::DatabaseError(e))
    }

    async fn update(&self, id: Uuid, other: UpdateTransaction) -> OptionalTransactionResult {
        const QUERY: &str = "
            UPDATE transactions SET 
                amount = $1, 
                category = $2 
            WHERE transaction_id = $3 returning *";

        sqlx::query_as(QUERY)
            .bind(other.amount)
            .bind(other.category)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| TransactionError::DatabaseError(e))
    }

    async fn delete(&self, id: Uuid) -> Result<HttpResponse, TransactionError> {
        const QUERY: &str = "DELETE from transactions WHERE transaction_id = $1";

        sqlx::query(QUERY)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| TransactionError::DatabaseError(e))?;


        Ok(HttpResponse::Accepted().finish())
    }
}
