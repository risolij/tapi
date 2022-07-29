use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum Category {
    BUSINESS,
    GAS,
    FOOD,
}
