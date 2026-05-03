//! Payment Module API Routes
//! 
//! REST endpoints for payment processing (VietQR, MoMo, ZaloPay)

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePaymentRequest {
    pub order_id: String,
    pub amount: i64,
    pub method: String,
    pub description: Option<String>,
    pub customer_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmPaymentRequest {
    pub provider_ref: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        // Payments
        .route("/payments", get(list_payments))
        .route("/payments", post(create_payment))
        .route("/payments/:id", get(get_payment))
        .route("/payments/:id/confirm", post(confirm_payment))
        .route("/payments/:id/cancel", post(cancel_payment))
        
        // VietQR
        .route("/vietqr/generate", post(generate_vietqr))
        .route("/vietqr/banks", get(list_banks))
        
        // Stats
        .route("/stats", get(get_payment_stats))
}

async fn list_payments(
    State(_state): State<AppState>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    Ok(Json(vec![]))
}

async fn get_payment(
    State(_state): State<AppState>,
    Path(payment_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "id": payment_id.to_string(),
        "status": "pending",
        "amount": 0,
        "method": "vietqr"
    })))
}

async fn create_payment(
    State(_state): State<AppState>,
    Json(req): Json<CreatePaymentRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let payment_id = Uuid::new_v4();
    
    Ok(Json(serde_json::json!({
        "id": payment_id.to_string(),
        "order_id": req.order_id,
        "amount": req.amount,
        "method": req.method,
        "status": "pending",
        "created_at": chrono::Utc::now().to_rfc3339()
    })))
}

async fn confirm_payment(
    State(_state): State<AppState>,
    Path(payment_id): Path<Uuid>,
    Json(_req): Json<ConfirmPaymentRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "id": payment_id.to_string(),
        "status": "completed",
        "confirmed_at": chrono::Utc::now().to_rfc3339()
    })))
}

async fn cancel_payment(
    State(_state): State<AppState>,
    Path(payment_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "id": payment_id.to_string(),
        "status": "cancelled",
        "cancelled_at": chrono::Utc::now().to_rfc3339()
    })))
}

async fn generate_vietqr(
    State(_state): State<AppState>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let order_id = req.get("order_id").and_then(|v| v.as_str()).unwrap_or("ORDER001");
    let amount = req.get("amount").and_then(|v| v.as_i64()).unwrap_or(100000);
    let description = req.get("description").and_then(|v| v.as_str()).unwrap_or("Thanh toan");
    
    // Generate VietQR payload
    let qr_data = format!(
        "00020101021138560010A0000007270124000066680000000000005802VN0104{}0112{}6304",
        order_id, amount
    );
    
    Ok(Json(serde_json::json!({
        "order_id": order_id,
        "amount": amount,
        "description": description,
        "qr_data": qr_data,
        "qr_image": format!("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg=="),
        "expires_at": chrono::Utc::now() + chrono::Duration::minutes(15)
    })))
}

async fn list_banks(
    State(_state): State<AppState>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    Ok(Json(vec![
        serde_json::json!({"id": "970436", "code": "BIDV", "name": "BIDV", "short_name": "BIDV"}),
        serde_json::json!({"id": "970405", "code": "VCB", "name": "Vietcombank", "short_name": "VCB"}),
        serde_json::json!({"id": "970418", "code": "CTG", "name": "VietinBank", "short_name": "CTG"}),
        serde_json::json!({"id": "970426", "code": "ACB", "name": "ACB", "short_name": "ACB"}),
        serde_json::json!({"id": "970448", "code": "TPB", "name": "TPBank", "short_name": "TPB"}),
        serde_json::json!({"id": "970454", "code": "MSB", "name": "Maritime Bank", "short_name": "MSB"}),
    ]))
}

async fn get_payment_stats(
    State(_state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "total_transactions": 0,
        "pending": 0,
        "completed": 0,
        "failed": 0,
        "total_amount": 0,
        "by_method": {
            "vietqr": 0,
            "momo": 0,
            "zalopay": 0,
            "cash": 0
        }
    })))
}
