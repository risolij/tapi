use sqlx::FromRow;

use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Copy, Clone, FromRow)]
pub struct Transaction {
    pub transaction_id: Uuid,
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub created: NaiveDateTime,
    pub amount: f32,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct PostTransaction {
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub amount: f32,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct UpdateTransaction {
    pub amount: f64
}
