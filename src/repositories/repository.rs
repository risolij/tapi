use async_trait::async_trait;
use uuid::Uuid;
use actix_web::HttpResponse;

#[async_trait]
pub trait Repository<T, P, U, E> {
    async fn post(&self, other: P) -> Result<T, E>;
    async fn get_one(&self, id: Uuid) -> Result<Option<T>, E>;
    async fn get_all(&self) -> Result<Vec<T>, E>;
    async fn update(&self, id: Uuid, other: U) -> Result<Option<T>, E>;
    async fn delete(&self, id: Uuid) -> Result<HttpResponse, E>;
}
