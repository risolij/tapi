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

#[async_trait]
impl Repository for TransactionRepository
{
    type Entity = Transaction;
    type Update = UpdateTransaction;
    type Insert = PostTransaction;
    type Error = TransactionError;

    async fn post(&self, other: Self::Insert) -> Result<Self::Entity, Self::Error> {
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

    async fn get_one(&self, id: Uuid) -> Result<Option<Self::Entity>, Self::Error> {
        sqlx::query_as(Transaction::sql_select_by_id())
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| TransactionError::DatabaseError(e))
    }

    async fn get_all(&self) -> Result<Vec<Self::Entity>, Self::Error> {
        sqlx::query_as(Transaction::sql_select())
            .fetch_all(&self.pool)
            .await
            .map_err(|e| TransactionError::DatabaseError(e))
    }

    async fn update(&self, id: Uuid, other: Self::Update) -> Result<Option<Self::Entity>, Self::Error> {
        sqlx::query_as(Transaction::sql_update())
            .bind(other.amount)
            .bind(other.category)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| TransactionError::DatabaseError(e))
    }

    async fn delete(&self, id: Uuid) -> Result<HttpResponse, Self::Error> {
        sqlx::query(Transaction::sql_delete())
            .bind(id)
            .execute(&self.pool)
            .await
            .and_then(|_| Ok(HttpResponse::Accepted().finish()))
            .map_err(|e| TransactionError::DatabaseError(e))
    }
}
