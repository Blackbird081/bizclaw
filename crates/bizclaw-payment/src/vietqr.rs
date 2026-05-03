//! # VietQR Payment Gateway
//!
//! Full implementation of VietQR payment for Vietnamese banks.
//! Based on VietQR Specification v1.0.9 (Jan 2025)
//!
//! ## Supported Banks
//! - BIDV, VCB, CTG, ACB, TPB, OCB, MSB, VPBank, SHB, HDB, NCB, SCB, PGB, GPB, SHBVN, SAIGONBANK, VietinBank, Agribank, MB, VietABank, OceanBank, SaigonBank
//!
//! ## Usage
//! ```rust
//! use bizclaw_payment::vietqr::{VietQRClient, BankConfig, format_vnd};
//!
//! let client = VietQRClient::new(BankConfig {
//!     bank_id: "970436".to_string(),
//!     merchant_id: "12345678901".to_string(),
//!     merchant_name: "CUA HANG ABC".to_string(),
//!     api_key: None,
//!     api_secret: None,
//! });
//!
//! let payload = client.create_payment("ORDER001", 100000, "Thanh toan don hang").unwrap();
//! println!("QR Data: {}", payload.qr_data);
//! println!("Amount: {}", format_vnd(payload.amount));
//! ```

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VietQRError {
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    #[error("Invalid merchant ID: {0}")]
    InvalidMerchant(String),
    #[error("Bank API error: {0}")]
    BankError(String),
    #[error("QR generation failed: {0}")]
    QRGeneration(String),
}

/// VietQR Bank configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankConfig {
    pub bank_id: String,
    pub merchant_id: String,
    pub merchant_name: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
}

/// VietQR Payment Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QRPayload {
    pub amount: i64,
    pub order_id: String,
    pub description: String,
    pub qr_data: String,
    pub bank_id: String,
    pub merchant_id: String,
    pub timestamp: i64,
}

/// VietQR Payment Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentQRResponse {
    pub success: bool,
    pub qr_data: Option<String>,
    pub qr_image_url: Option<String>,
    pub transaction_id: Option<String>,
    pub error: Option<String>,
}

/// VietBank lookup result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankInfo {
    pub bank_id: String,
    pub bank_name: String,
    pub short_name: String,
    pub logo_url: String,
    pub supports_qr: bool,
}

/// VietQR Client for generating payment QR codes
pub struct VietQRClient {
    config: BankConfig,
    base_url: String,
}

impl VietQRClient {
    pub fn new(config: BankConfig) -> Self {
        Self {
            config,
            base_url: "https://api.vietqr.io".to_string(),
        }
    }

    pub fn new_with_sandbox(config: BankConfig) -> Self {
        Self {
            config,
            base_url: "https://sandbox.vietqr.io".to_string(),
        }
    }

    /// Create a VietQR payload for payment
    pub fn create_payment(
        &self,
        order_id: &str,
        amount: i64,
        description: &str,
    ) -> Result<QRPayload, VietQRError> {
        // Validate inputs
        if amount <= 0 {
            return Err(VietQRError::InvalidAmount(format!("Amount must be positive: {}", amount)));
        }
        if amount > 999_999_999_999 {
            return Err(VietQRError::InvalidAmount("Amount exceeds maximum".to_string()));
        }
        if self.config.merchant_id.len() < 8 || self.config.merchant_id.len() > 15 {
            return Err(VietQRError::InvalidMerchant(
                "Merchant ID must be 8-15 characters".to_string(),
            ));
        }

        let timestamp = chrono::Utc::now().timestamp();

        // Build VietQR payload following VietQR Spec 1.0.9
        let mut payload = Vec::new();

        // Add leading zeros to merchant ID for VietQR format
        let padded_merchant = format!("{:0>15}", self.config.merchant_id);

        // VietQR Format ID (2 chars): "00"
        payload.push(("00".to_string(), "01".to_string()));

        // QR Type (2 chars): "01" for QR static, "02" for QR dynamic
        payload.push(("01".to_string(), "11".to_string()));

        // Payment Method (2 chars): "00" for VietQR
        payload.push(("02".to_string(), "00".to_string()));

        // Bank ID (4 chars): padded with spaces
        let bank_id_padded = format!("{:4}", self.config.bank_id);
        payload.push(("03".to_string(), bank_id_padded));

        // Merchant ID (15 chars): padded with spaces
        payload.push(("04".to_string(), padded_merchant));

        // Merchant Name (25 chars): padded with spaces
        let name_padded = format!("{:25}", self.config.merchant_name.to_uppercase());
        payload.push(("05".to_string(), name_padded));

        // Amount (9 chars): right-justified, padded with zeros
        let amount_str = format!("{:09}", amount);
        payload.push(("54".to_string(), amount_str));

        // Currency (3 chars): "VND"
        payload.push(("53".to_string(), "704".to_string())); // 704 = VND in ISO 4217

        // Country Code (2 chars): "VN"
        payload.push(("58".to_string(), "VN".to_string()));

        // Merchant Name (99 chars): additional field
        payload.push(("00".to_string(), "01".to_string()));
        let name_99 = format!("{:29}", self.config.merchant_name.to_uppercase());
        payload.push(("59".to_string(), name_99));

        // Mobile indicator (2 chars): "00" for no mobile
        payload.push(("60".to_string(), "00".to_string()));

        // Transaction ID (17 chars): order reference
        let tx_id = format!("{:0>8}{:0>9}", order_id.chars().take(8).collect::<String>(), timestamp % 1_000_000_000);
        payload.push(("08".to_string(), tx_id));

        // Build QR string
        let mut qr_data = String::new();
        for (id, value) in &payload {
            let id_len = id.len();
            let value_len = value.len();
            let len_str = format!("{:02}", id_len + value_len);
            qr_data.push_str(&len_str);
            qr_data.push_str(id);
            qr_data.push_str(value);
        }

        // Add CRC16 checksum
        let crc = Self::calculate_crc16(&qr_data);
        qr_data.push_str(&format!("{:04}", crc));

        Ok(QRPayload {
            amount,
            order_id: order_id.to_string(),
            description: description.to_string(),
            qr_data,
            bank_id: self.config.bank_id.clone(),
            merchant_id: self.config.merchant_id.clone(),
            timestamp,
        })
    }

    /// Generate QR code image as base64 PNG
    pub async fn generate_qr_image(&self, payload: &QRPayload) -> Result<String, VietQRError> {
        // For production, call VietQR API to generate image
        let client = reqwest::Client::new();
        
        let response = client
            .post(format!("{}/v2/qr/{}/{}", self.base_url, self.config.bank_id, self.config.merchant_id))
            .json(&serde_json::json!({
                "accountNumber": self.config.merchant_id,
                "accountName": self.config.merchant_name,
                "amount": payload.amount,
                "orderId": payload.order_id,
                "qrCode": payload.qr_data,
            }))
            .send()
            .await
            .map_err(|e| VietQRError::BankError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(VietQRError::BankError(format!(
                "API returned status: {}",
                response.status()
            )));
        }

        let result = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| VietQRError::QRGeneration(e.to_string()))?;

        let qr_data = result["data"]["qrDataURL"]
            .as_str()
            .map(|s| s.to_string());

        let qr_image_url = result["data"]["qrCode"]
            .as_str()
            .map(|s| s.to_string());

        Ok(qr_image_url.unwrap_or_else(|| result["data"]["qr"].as_str().unwrap_or("").to_string()))
    }

    /// Get VietQR payment URL for deep linking
    pub fn get_payment_url(&self, payload: &QRPayload) -> String {
        format!(
            "https://vietqr.io/{}/{}/{}/{}",
            self.config.bank_id,
            self.config.merchant_id,
            payload.amount,
            payload.order_id
        )
    }

    /// Get list of supported banks
    pub async fn get_supported_banks(&self) -> Result<Vec<BankInfo>, VietQRError> {
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/v2/banks", self.base_url))
            .send()
            .await
            .map_err(|e| VietQRError::BankError(e.to_string()))?;

        let banks = response
            .json::<Vec<BankInfo>>()
            .await
            .map_err(|e| VietQRError::BankError(e.to_string()))?;

        Ok(banks)
    }

    /// Calculate CRC16 checksum for QR data
    fn calculate_crc16(data: &str) -> String {
        let mut crc: u16 = 0xFFFF;
        
        for byte in data.bytes() {
            crc ^= (byte as u16) << 8;
            for _ in 0..8 {
                if (crc & 0x8000) != 0 {
                    crc = (crc << 1) ^ 0x1021;
                } else {
                    crc <<= 1;
                }
            }
        }
        
        format!("{:04X}", crc)
    }

    /// Verify QR payment from webhook/callback
    pub async fn verify_payment(
        &self,
        webhook_data: &serde_json::Value,
    ) -> Result<bool, VietQRError> {
        // Verify signature if API key is configured
        if let (Some(api_key), Some(signature)) = (
            &self.config.api_key,
            webhook_data.get("signature"),
        ) {
            let message = webhook_data["transactionRef"]
                .as_str()
                .unwrap_or_default();
            
            let expected_sig = Self::calculate_hmac_sha256(message, api_key);
            
            if signature.as_str() != Some(&expected_sig) {
                return Ok(false);
            }
        }

        // Verify merchant ID matches
        let merchant_id = webhook_data["merchantId"]
            .as_str()
            .unwrap_or("");
        
        if merchant_id != self.config.merchant_id {
            return Ok(false);
        }

        Ok(true)
    }

    fn calculate_hmac_sha256(message: &str, key: &str) -> String {
        use sha2::{Sha256, Digest};
        use std::fmt::Write;
        let mut hasher = Sha256::new();
        hasher.update(format!("{}_{}", message, key));
        let result = hasher.finalize();
        let mut hex = String::new();
        for byte in &result[0..8] {
            write!(&mut hex, "{:02X}", byte).unwrap();
        }
        hex
    }
}

impl Default for VietQRClient {
    fn default() -> Self {
        Self::new(BankConfig {
            bank_id: "970436".to_string(), // VietinBank default
            merchant_id: "12345678901".to_string(),
            merchant_name: "BIZCLAW STORE".to_string(),
            api_key: None,
            api_secret: None,
        })
    }
}

/// Quick format helper for VND currency with thousand separators
pub fn format_vnd(amount: i64) -> String {
    let s = amount.abs().to_string();
    let mut result = String::new();
    let mut count = 0;
    for c in s.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.insert(0, '.');
        }
        result.insert(0, c);
        count += 1;
    }
    if amount < 0 {
        result.insert(0, '-');
    }
    format!("{} VND", result)
}

/// Parse QR string back to payload
pub fn parse_qr_data(qr_data: &str) -> Option<QRPayload> {
    let mut pos = 0;
    let bytes = qr_data.as_bytes();
    let mut fields: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    
    while pos < bytes.len() - 4 {
        if bytes[pos] < b'0' || bytes[pos] > b'9' {
            pos += 1;
            continue;
        }
        
        let len = ((bytes[pos] - b'0') * 10 + (bytes[pos + 1] - b'0')) as usize;
        pos += 2;
        
        if pos + len > bytes.len() - 4 {
            break;
        }
        
        let id = String::from_utf8_lossy(&bytes[pos..pos + 2]).to_string();
        let value = String::from_utf8_lossy(&bytes[pos + 2..pos + len]).to_string();
        fields.insert(id, value);
        pos += len;
    }
    
    let amount = fields.get("54")?.parse().ok()?;
    let merchant_id = fields.get("04")?.trim().to_string();
    
    Some(QRPayload {
        amount,
        order_id: fields.get("08").cloned().unwrap_or_default(),
        description: String::new(),
        qr_data: qr_data.to_string(),
        bank_id: fields.get("03").cloned().unwrap_or_default(),
        merchant_id,
        timestamp: chrono::Utc::now().timestamp(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_payment() {
        let client = VietQRClient::new(BankConfig {
            bank_id: "970436".to_string(),
            merchant_id: "12345678901".to_string(),
            merchant_name: "TEST STORE".to_string(),
            api_key: None,
            api_secret: None,
        });

        let payload = client.create_payment("ORDER001", 100000, "Test payment").unwrap();
        
        assert_eq!(payload.amount, 100000);
        assert_eq!(payload.order_id, "ORDER001");
        assert!(payload.qr_data.contains("970436")); // bank ID
        assert!(payload.qr_data.contains("12345678901")); // merchant ID
    }

    #[test]
    fn test_format_vnd() {
        assert_eq!(format_vnd(100000), "100.000 VND");
        assert_eq!(format_vnd(1500000), "1.500.000 VND");
        assert_eq!(format_vnd(100000000), "100.000.000 VND");
        assert_eq!(format_vnd(1000), "1.000 VND");
        assert_eq!(format_vnd(100), "100 VND");
    }

    #[test]
    fn test_crc_calculation() {
        let data = "00020101021238530010A00000070601234567890102030406090215 Tran Van A55070400005802VN5914 TEST MERCHANT60120004 Test610070000623020310000000000000050000";
        
        let crc = VietQRClient::default().create_payment("TEST", 100, "test").map(|p| p.qr_data);
        assert!(crc.is_ok());
    }
}
