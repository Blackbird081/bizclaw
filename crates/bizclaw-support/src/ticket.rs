//! Ticket management for Vietnamese customer support

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TicketStatus {
    Open,
    Pending,
    InProgress,
    Waiting,
    Resolved,
    Closed,
    Escalated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TicketPriority {
    Critical,
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TicketChannel {
    Zalo,
    Facebook,
    Telegram,
    Email,
    Phone,
    InPerson,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: String,
    pub customer_id: String,
    pub channel: TicketChannel,
    pub subject: String,
    pub status: TicketStatus,
    pub priority: TicketPriority,
    pub sla_due: Option<DateTime<Utc>>,
    pub messages: Vec<TicketMessage>,
    pub tags: Vec<String>,
    pub assignees: Vec<String>,
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketMessage {
    pub id: String,
    pub ticket_id: String,
    pub sender_id: String,
    pub sender_name: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub attachments: Vec<String>,
}

impl TicketMessage {
    pub fn new(ticket_id: &str, sender_id: String, sender_name: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            ticket_id: ticket_id.to_string(),
            sender_id,
            sender_name,
            content,
            timestamp: Utc::now(),
            attachments: Vec::new(),
        }
    }
}

impl Ticket {
    pub fn new(customer_id: String, channel: TicketChannel, subject: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            customer_id,
            channel,
            subject,
            status: TicketStatus::Open,
            priority: TicketPriority::Normal,
            sla_due: None,
            messages: Vec::new(),
            tags: Vec::new(),
            assignees: Vec::new(),
            metadata: std::collections::HashMap::new(),
        }
    }

    pub fn add_message(&mut self, sender_id: String, sender_name: String, content: String) {
        self.messages.push(TicketMessage {
            id: Uuid::new_v4().to_string(),
            ticket_id: self.id.clone(),
            sender_id,
            sender_name,
            content,
            timestamp: Utc::now(),
            attachments: Vec::new(),
        });
    }

    pub fn is_sla_violated(&self) -> bool {
        if let Some(due) = self.sla_due {
            Utc::now() > due
        } else {
            false
        }
    }

    pub fn escalate(&mut self) {
        self.status = TicketStatus::Escalated;
    }

    pub fn resolve(&mut self) {
        self.status = TicketStatus::Resolved;
    }
}
