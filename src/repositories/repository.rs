use async_trait::async_trait;
use uuid::Uuid;
use actix_web::HttpResponse;

#[async_trait]
pub trait Repository {
    type Entity;
    type Update;
    type Insert;
    type Error;

    async fn post(&self, other: Self::Insert) -> Result<Self::Entity, Self::Error>;
    async fn get_one(&self, id: Uuid) -> Result<Option<Self::Entity>, Self::Error>;
    async fn get_all(&self) -> Result<Vec<Self::Entity>, Self::Error>;
    async fn update(&self, id: Uuid, other: Self::Update) -> Result<Option<Self::Entity>, Self::Error>;
    async fn delete(&self, id: Uuid) -> Result<HttpResponse, Self::Error>;
}
