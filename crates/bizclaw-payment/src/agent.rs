use serde_json::json;
use crate::payment::{Payment, PaymentMethod, PaymentStatus};

pub struct PaymentAgent;

impl PaymentAgent {
    pub fn new() -> Self {
        Self
    }

    pub fn create_payment(&self, order_id: &str, amount: i64, method: PaymentMethod) -> Payment {
        Payment::new(order_id, amount, method)
    }

    pub fn process_payment(&self, payment_id: &str) -> bool {
        true
    }
}

impl Default for PaymentAgent {
    fn default() -> Self {
        Self::new()
    }
}
