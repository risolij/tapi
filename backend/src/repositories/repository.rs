use async_trait::async_trait;
use actix_web::HttpResponse;

#[async_trait]
pub trait Repository {
    type Entity;
    type Update;
    type Insert;
    type Error;
    type Id;

    async fn post(&self, other: Self::Insert) -> Result<Self::Entity, Self::Error>;
    async fn get_one(&self, id: Self::Id) -> Result<Option<Self::Entity>, Self::Error>;
    async fn get_all(&self) -> Result<Vec<Self::Entity>, Self::Error>;
    async fn update(&self, id: Self::Id, other: Self::Update) -> Result<Option<Self::Entity>, Self::Error>;
    async fn delete(&self, id: Self::Id) -> Result<HttpResponse, Self::Error>;
}
