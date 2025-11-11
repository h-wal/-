use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Trade {
    pub buyer: String,
    pub seller: String,
    pub qty: u64,
    pub price: u64,
}

