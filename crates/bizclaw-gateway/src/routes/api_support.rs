//! Support Module API Routes
//! 
//! REST endpoints for ticket management, customer support

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
pub struct CreateTicketRequest {
    pub customer_id: String,
    pub channel: String,
    pub subject: String,
    pub priority: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddMessageRequest {
    pub sender_id: String,
    pub sender_name: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

// GET /api/support/tickets
async fn list_tickets(
    State(state): State<AppState>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    // In production, query from database
    // For now, return empty list
    Ok(Json(vec![]))
}

// GET /api/support/tickets/:id
async fn get_ticket(
    State(_state): State<AppState>,
    Path(ticket_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "id": ticket_id,
        "status": "open",
        "message": "Ticket retrieved"
    })))
}

// POST /api/support/tickets
async fn create_ticket(
    State(_state): State<AppState>,
    Json(req): Json<CreateTicketRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let ticket_id = Uuid::new_v4().to_string();
    Ok(Json(serde_json::json!({
        "id": ticket_id,
        "customer_id": req.customer_id,
        "channel": req.channel,
        "subject": req.subject,
        "status": "open",
        "priority": req.priority.unwrap_or_else(|| "normal".to_string()),
        "created_at": chrono::Utc::now().to_rfc3339()
    })))
}

// POST /api/support/tickets/:id/messages
async fn add_message(
    State(_state): State<AppState>,
    Path(ticket_id): Path<String>,
    Json(req): Json<AddMessageRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let message_id = Uuid::new_v4().to_string();
    Ok(Json(serde_json::json!({
        "id": message_id,
        "ticket_id": ticket_id,
        "sender_id": req.sender_id,
        "sender_name": req.sender_name,
        "content": req.content,
        "created_at": chrono::Utc::now().to_rfc3339()
    })))
}

// PUT /api/support/tickets/:id/status
async fn update_status(
    State(_state): State<AppState>,
    Path(ticket_id): Path<String>,
    Json(req): Json<UpdateStatusRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "id": ticket_id,
        "status": req.status,
        "updated_at": chrono::Utc::now().to_rfc3339()
    })))
}

// DELETE /api/support/tickets/:id
async fn delete_ticket(
    State(_state): State<AppState>,
    Path(_ticket_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}

// GET /api/support/stats
async fn get_stats(
    State(_state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "total_tickets": 0,
        "open_tickets": 0,
        "resolved_tickets": 0,
        "avg_response_time": "0h",
        "sla_compliance": 100.0
    })))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/tickets", get(list_tickets))
        .route("/tickets", post(create_ticket))
        .route("/tickets/:id", get(get_ticket))
        .route("/tickets/:id", put(update_status))
        .route("/tickets/:id", delete(delete_ticket))
        .route("/tickets/:id/messages", post(add_message))
        .route("/stats", get(get_stats))
}
