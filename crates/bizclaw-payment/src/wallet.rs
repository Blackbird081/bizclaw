use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletPayment {
    pub phone: String,
    pub amount: i64,
    pub order_id: String,
    pub description: String,
}

pub struct MoMoClient {
    pub partner_code: String,
    pub secret_key: String,
    pub access_key: String,
}

impl MoMoClient {
    pub fn new(partner_code: &str, secret_key: &str, access_key: &str) -> Self {
        Self {
            partner_code: partner_code.to_string(),
            secret_key: secret_key.to_string(),
            access_key: access_key.to_string(),
        }
    }

    pub fn create_payment_request(&self, payment: &WalletPayment) -> anyhow::Result<String> {
        Ok(format!("MoMo payment: {} - {} VND", payment.phone, payment.amount))
    }
}

pub struct ZaloPayClient {
    pub app_id: String,
    pub key1: String,
    pub key2: String,
}

impl ZaloPayClient {
    pub fn new(app_id: &str, key1: &str, key2: &str) -> Self {
        Self {
            app_id: app_id.to_string(),
            key1: key1.to_string(),
            key2: key2.to_string(),
        }
    }

    pub fn create_payment(&self, payment: &WalletPayment) -> anyhow::Result<String> {
        Ok(format!("ZaloPay: {} - {} VND", payment.phone, payment.amount))
    }
}
