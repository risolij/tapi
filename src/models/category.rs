use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum Category {
    BUSINESS,
    GAS,
    FOOD,
}
