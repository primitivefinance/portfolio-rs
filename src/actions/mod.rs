use serde::{Deserialize, Serialize};

pub mod swap;

/// Actions that can be performed on a Portfolio contract.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Actions {
    Swap,
}
