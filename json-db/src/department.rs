use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Department {
    id: u64,
    name: String,
    employee_count: u32,
    budget: f64,
}
impl Department {
    pub fn new(name: String, employee_count: u32, budget: f64) -> Self {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Department {
            id,
            name,
            employee_count,
            budget,
        }
    }
}
