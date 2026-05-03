//! Inventory Module API Routes
//! 
//! REST endpoints for stock management

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
    routing::{get, post, put, delete},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateItemRequest {
    pub sku: String,
    pub name: String,
    pub category: Option<String>,
    pub cost: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdjustStockRequest {
    pub quantity: i32,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReserveStockRequest {
    pub quantity: i32,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        // Items
        .route("/items", get(list_items))
        .route("/items", post(create_item))
        .route("/items/:id", get(get_item))
        .route("/items/:id", put(update_item))
        .route("/items/:id", delete(delete_item))
        
        // Stock operations
        .route("/items/:id/receive", post(receive_stock))
        .route("/items/:id/sell", post(sell_stock))
        .route("/items/:id/reserve", post(reserve_stock))
        .route("/items/:id/release", post(release_stock))
        
        // Alerts
        .route("/alerts/low-stock", get(get_low_stock_alerts))
        
        // Warehouses
        .route("/warehouses", get(list_warehouses))
        .route("/warehouses", post(create_warehouse))
        
        // Stats
        .route("/stats", get(get_inventory_stats))
}

async fn list_items(
    State(_state): State<AppState>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    Ok(Json(vec![]))
}

async fn get_item(
    State(_state): State<AppState>,
    Path(item_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "id": item_id,
        "sku": "SKU001",
        "name": "Sample Item",
        "quantity": 100,
        "available": 100,
        "reserved": 0,
        "cost": 50000,
        "value": 5000000
    })))
}

async fn create_item(
    State(_state): State<AppState>,
    Json(req): Json<CreateItemRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let item_id = Uuid::new_v4().to_string();
    Ok(Json(serde_json::json!({
        "id": item_id,
        "sku": req.sku,
        "name": req.name,
        "category": req.category.unwrap_or_default(),
        "cost": req.cost,
        "quantity": 0,
        "available": 0,
        "reserved": 0,
        "created_at": chrono::Utc::now().to_rfc3339()
    })))
}

async fn update_item(
    State(_state): State<AppState>,
    Path(item_id): Path<String>,
    Json(_req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "id": item_id,
        "updated_at": chrono::Utc::now().to_rfc3339()
    })))
}

async fn delete_item(
    State(_state): State<AppState>,
    Path(_item_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}

async fn receive_stock(
    State(_state): State<AppState>,
    Path(item_id): Path<String>,
    Json(req): Json<AdjustStockRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "item_id": item_id,
        "quantity_added": req.quantity,
        "reason": req.reason,
        "new_quantity": req.quantity,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

async fn sell_stock(
    State(_state): State<AppState>,
    Path(item_id): Path<String>,
    Json(req): Json<AdjustStockRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "item_id": item_id,
        "quantity_sold": req.quantity,
        "reason": req.reason,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

async fn reserve_stock(
    State(_state): State<AppState>,
    Path(item_id): Path<String>,
    Json(req): Json<ReserveStockRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "item_id": item_id,
        "quantity_reserved": req.quantity,
        "success": true
    })))
}

async fn release_stock(
    State(_state): State<AppState>,
    Path(item_id): Path<String>,
    Json(req): Json<ReserveStockRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "item_id": item_id,
        "quantity_released": req.quantity,
        "success": true
    })))
}

async fn get_low_stock_alerts(
    State(_state): State<AppState>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    Ok(Json(vec![]))
}

async fn list_warehouses(
    State(_state): State<AppState>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    Ok(Json(vec![serde_json::json!({
        "id": Uuid::new_v4().to_string(),
        "code": "MAIN",
        "name": "Kho chính",
        "is_default": true
    })]))
}

async fn create_warehouse(
    State(_state): State<AppState>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let warehouse_id = Uuid::new_v4();
    Ok(Json(serde_json::json!({
        "id": warehouse_id.to_string(),
        "code": req.get("code").and_then(|v| v.as_str()).unwrap_or("WH001"),
        "name": req.get("name").and_then(|v| v.as_str()).unwrap_or("Warehouse"),
        "created_at": chrono::Utc::now().to_rfc3339()
    })))
}

async fn get_inventory_stats(
    State(_state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "total_items": 0,
        "total_value": 0,
        "low_stock_count": 0,
        "out_of_stock_count": 0
    })))
}
