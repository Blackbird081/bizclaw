//! Accounting Module API Routes
//! 
//! REST endpoints for bookkeeping and financial reports

use std::sync::Arc;
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
pub struct RecordTransactionRequest {
    pub description: String,
    pub entries: Vec<TransactionEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionEntry {
    pub account_code: String,
    pub debit: i64,
    pub credit: i64,
    pub memo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordSaleRequest {
    pub amount: i64,
    pub customer_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordExpenseRequest {
    pub amount: i64,
    pub category: String,
    pub description: Option<String>,
}

pub async fn list_transactions(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    Ok(Json(vec![]))
}

pub async fn get_transaction(
    State(_state): State<Arc<AppState>>,
    Path(tx_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "id": tx_id,
        "description": "Sample transaction",
        "entries": []
    })))
}

pub async fn record_transaction(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<RecordTransactionRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let tx_id = Uuid::new_v4().to_string();
    Ok(Json(serde_json::json!({
        "id": tx_id,
        "description": req.description,
        "entries": req.entries,
        "created_at": chrono::Utc::now().to_rfc3339()
    })))
}

pub async fn record_sale(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<RecordSaleRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let tx_id = Uuid::new_v4().to_string();
    let vat = req.amount * 10 / 100;
    let total = req.amount + vat;
    
    Ok(Json(serde_json::json!({
        "id": tx_id,
        "type": "sale",
        "amount": req.amount,
        "vat": vat,
        "total": total,
        "customer_id": req.customer_id,
        "created_at": chrono::Utc::now().to_rfc3339(),
        "entries": [
            {"account": "1200", "debit": total, "credit": 0},
            {"account": "5000", "debit": 0, "credit": req.amount},
            {"account": "2200", "debit": 0, "credit": vat}
        ]
    })))
}

pub async fn record_purchase(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let tx_id = Uuid::new_v4().to_string();
    let amount = req.get("amount").and_then(|v| v.as_i64()).unwrap_or(0);
    let vat = amount * 10 / 100;
    let total = amount + vat;
    
    Ok(Json(serde_json::json!({
        "id": tx_id,
        "type": "purchase",
        "amount": amount,
        "vat": vat,
        "total": total,
        "created_at": chrono::Utc::now().to_rfc3339()
    })))
}

pub async fn record_expense(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<RecordExpenseRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let tx_id = Uuid::new_v4().to_string();
    let account = match req.category.as_str() {
        "selling" => "6100",
        "admin" => "6200",
        _ => "6200",
    };
    
    Ok(Json(serde_json::json!({
        "id": tx_id,
        "type": "expense",
        "amount": req.amount,
        "category": req.category,
        "account": account,
        "description": req.description,
        "created_at": chrono::Utc::now().to_rfc3339()
    })))
}

pub async fn balance_sheet(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "date": chrono::Utc::now().format("%Y-%m-%d").to_string(),
        "assets": 0,
        "liabilities": 0,
        "equity": 0,
        "balanced": true
    })))
}

pub async fn income_statement(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "period": chrono::Utc::now().format("%Y-%m").to_string(),
        "revenue": 0,
        "cogs": 0,
        "gross_profit": 0,
        "opex": 0,
        "net_profit": 0
    })))
}

pub async fn trial_balance(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "date": chrono::Utc::now().format("%Y-%m-%d").to_string(),
        "accounts": [],
        "total_debit": 0,
        "total_credit": 0,
        "balanced": true
    })))
}

pub async fn cash_flow(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "period": chrono::Utc::now().format("%Y-%m").to_string(),
        "operating": 0,
        "investing": 0,
        "financing": 0,
        "net_cash": 0
    })))
}

pub async fn vat_report(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "period": chrono::Utc::now().format("%Y-%m").to_string(),
        "output_vat": 0,
        "input_vat": 0,
        "vat_payable": 0
    })))
}

pub async fn list_accounts(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    Ok(Json(vec![
        serde_json::json!({"code": "1100", "name": "Tiền mặt", "type": "asset"}),
        serde_json::json!({"code": "1200", "name": "Phải thu", "type": "asset"}),
        serde_json::json!({"code": "1500", "name": "Hàng tồn kho", "type": "asset"}),
        serde_json::json!({"code": "2100", "name": "Phải trả", "type": "liability"}),
        serde_json::json!({"code": "2200", "name": "Thuế phải nộp", "type": "liability"}),
        serde_json::json!({"code": "5000", "name": "Doanh thu", "type": "revenue"}),
        serde_json::json!({"code": "6000", "name": "Giá vốn", "type": "expense"}),
        serde_json::json!({"code": "6100", "name": "Chi phí bán hàng", "type": "expense"}),
        serde_json::json!({"code": "6200", "name": "Chi phí quản lý", "type": "expense"}),
    ]))
}
