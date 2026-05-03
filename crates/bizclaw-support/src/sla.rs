//! SLA management for Vietnamese customer support
//! 
//! ## SLA Tiers
//! - VIP: 1 hour response
//! - Express: 4 hours
//! - Normal: 8 hours
//! - Economy: 24 hours

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use super::ticket::Ticket;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SlaTier {
    Vip,     // 1 hour
    Express,  // 4 hours
    Normal,   // 8 hours
    Economy,  // 24 hours
}

impl SlaTier {
    pub fn response_time(&self) -> Duration {
        match self {
            SlaTier::Vip => Duration::hours(1),
            SlaTier::Express => Duration::hours(4),
            SlaTier::Normal => Duration::hours(8),
            SlaTier::Economy => Duration::hours(24),
        }
    }

    pub fn next_deadline(&self) -> chrono::DateTime<Utc> {
        Utc::now() + self.response_time()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlaConfig {
    pub tier: SlaTier,
    pub business_hours_only: bool,
    pub auto_escalate: bool,
}

impl Default for SlaConfig {
    fn default() -> Self {
        Self {
            tier: SlaTier::Normal,
            business_hours_only: false,
            auto_escalate: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlaViolation {
    pub ticket_id: String,
    pub sla_tier: SlaTier,
    pub due_at: chrono::DateTime<Utc>,
    pub breached_at: chrono::DateTime<Utc>,
    pub resolution: Option<String>,
}

impl SlaViolation {
    pub fn check(ticket: &Ticket, tier: &SlaTier) -> Option<Self> {
        let due = tier.next_deadline();
        if Utc::now() > due {
            Some(SlaViolation {
                ticket_id: ticket.id.clone(),
                sla_tier: tier.clone(),
                due_at: due,
                breached_at: Utc::now(),
                resolution: None,
            })
        } else {
            None
        }
    }
}
