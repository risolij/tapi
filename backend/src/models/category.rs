use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Display, Debug, Serialize, Deserialize, Copy, Clone)]
#[sqlx(type_name = "CATEGORY")]
pub enum Category {
    Business,
    Gas,
    Food,
}
