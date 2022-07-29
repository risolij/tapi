use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Repository<T, P, E>: {
    async fn post(&self, other: P) -> Result<T, E>;
    async fn get_one(&self, id: Uuid) -> Result<Option<T>, E>;
    async fn get_all(&self) -> Result<Vec<T>, E>;
}
