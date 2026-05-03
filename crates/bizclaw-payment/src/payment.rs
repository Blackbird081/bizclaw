use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: String,
    pub order_id: String,
    pub amount: i64,
    pub currency: String,
    pub method: PaymentMethod,
    pub status: PaymentStatus,
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    VietQR,
    MoMo,
    ZaloPay,
    Cash,
    BankTransfer,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
}

impl Payment {
    pub fn new(order_id: &str, amount: i64, method: PaymentMethod) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            order_id: order_id.to_string(),
            amount,
            currency: "VND".to_string(),
            method,
            status: PaymentStatus::Pending,
            customer_name: None,
            customer_phone: None,
            created_at: Utc::now(),
            paid_at: None,
        }
    }

    pub fn mark_paid(&mut self) {
        self.status = PaymentStatus::Completed;
        self.paid_at = Some(Utc::now());
    }
}
