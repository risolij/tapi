use sqlx::FromRow;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::Component;
use uuid::Uuid;
use crate::models::category::Category;
use crate::lib::Schema;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, FromRow, Component)]
pub struct Transaction {
    pub transaction_id: Uuid,
    pub user_id: Uuid,
    pub account_id: Uuid,
    #[component(value_type = String, format = ComponentFormat::DateTime)]
    pub created: NaiveDateTime,
    #[component(value_type = String)]
    pub category: Category,
    pub amount: f32,
}

impl Schema for Transaction {
    type Id = Uuid;

    fn sql_id(&self) -> Self::Id {
        self.transaction_id
    }

    fn sql_select() -> &'static str {
        "SELECT * FROM transactions"
    }

    fn sql_select_by_id() -> &'static str {
        "SELECT * FROM transactions WHERE transaction_id = $1"
    }

    fn sql_insert() -> &'static str {
        "INSERT INTO transactions 
            ( user_id, account_id, amount, created, category ) 
        VALUES 
            ( $1, $2, $3, $4, $5 ) 
        returning *"
    }

    fn sql_update() -> &'static str {
        "UPDATE transactions SET 
            amount = $1, 
            category = $2 
        WHERE transaction_id = $3 returning *"
    }

    fn sql_delete() -> &'static str {
        "DELETE from transactions WHERE transaction_id = $1"
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Component)]
pub struct PostTransaction {
    pub user_id: Uuid,
    pub account_id: Uuid,
    #[component(value_type = String)]
    pub category: Category,
    pub amount: f32,
}

#[derive(Serialize, Deserialize, Copy, Clone, Component)]
pub struct UpdateTransaction {
    pub amount: f64,
    #[component(value_type = String)]
    pub category: Category,
}
