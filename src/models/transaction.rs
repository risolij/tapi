use sqlx::FromRow;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::Component;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, FromRow, Component)]
pub struct Transaction {
    pub transaction_id: Uuid,
    pub user_id: Uuid,
    pub account_id: Uuid,
    #[component(value_type = String, format = ComponentFormat::DateTime)]
    pub created: NaiveDateTime,
    pub amount: f32,
}

#[derive(Serialize, Deserialize, Copy, Clone, Component)]
pub struct PostTransaction {
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub amount: f32,
}

#[derive(Serialize, Deserialize, Copy, Clone, Component)]
pub struct UpdateTransaction {
    pub amount: f64,
}
