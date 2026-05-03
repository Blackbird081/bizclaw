//! BizClaw Support Agent - Customer ticket system for Vietnamese OPC
//! 
//! ## Features
//! - Ticket management with SLA tracking
//! - Customer 360 profile
//! - Auto-assignment by keywords
//! - Multi-channel (Zalo, Facebook, Telegram)
//! - Sentiment detection
//! - Escalation rules
//! - Full CRUD operations

pub mod ticket;
pub mod customer;
pub mod sla;

pub use ticket::{Ticket, TicketStatus, TicketPriority, TicketChannel, TicketMessage};
pub use customer::{Customer, CustomerSegment, CustomerStats};
pub use sla::{SlaTier, SlaConfig, SlaViolation};

pub struct SupportAgent {
    pub config: SupportConfig,
    pub tickets: std::collections::HashMap<String, Ticket>,
    pub customers: std::collections::HashMap<String, Customer>,
    pub ticket_counter: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SupportConfig {
    pub auto_assign: bool,
    pub sentiment_threshold: f32,
    pub escalation_rules: Vec<EscalationRule>,
    pub default_sla: SlaTier,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EscalationRule {
    pub trigger_keywords: Vec<String>,
    pub priority: TicketPriority,
    pub assign_to: Option<String>,
    pub sla_override: Option<SlaTier>,
}

impl Default for SupportConfig {
    fn default() -> Self {
        Self {
            auto_assign: true,
            sentiment_threshold: 0.3,
            escalation_rules: vec![
                EscalationRule {
                    trigger_keywords: vec![
                        "hủy đơn".into(),
                        "hoàn tiền".into(),
                        "khiếu nại".into(),
                        "tố cáo".into(),
                        "gọi luật sư".into(),
                    ],
                    priority: TicketPriority::Critical,
                    assign_to: Some("escalation_team".into()),
                    sla_override: Some(SlaTier::Vip),
                },
                EscalationRule {
                    trigger_keywords: vec![
                        "chậm giao hàng".into(),
                        "sản phẩm lỗi".into(),
                        "sai sản phẩm".into(),
                        "đổi trả".into(),
                    ],
                    priority: TicketPriority::High,
                    assign_to: Some("quality_team".into()),
                    sla_override: Some(SlaTier::Express),
                },
            ],
            default_sla: SlaTier::Normal,
        }
    }
}

impl SupportAgent {
    pub fn new(config: SupportConfig) -> Self {
        Self {
            config,
            tickets: std::collections::HashMap::new(),
            customers: std::collections::HashMap::new(),
            ticket_counter: 0,
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(SupportConfig::default())
    }

    // Customer CRUD
    pub fn create_customer(&mut self, name: String) -> Customer {
        let customer = Customer::new(name);
        self.customers.insert(customer.id.clone(), customer.clone());
        customer
    }

    pub fn get_customer(&self, customer_id: &str) -> Option<&Customer> {
        self.customers.get(customer_id)
    }

    pub fn get_customer_mut(&mut self, customer_id: &str) -> Option<&mut Customer> {
        self.customers.get_mut(customer_id)
    }

    pub fn list_customers(&self) -> Vec<&Customer> {
        self.customers.values().collect()
    }

    pub fn update_customer(&mut self, customer_id: &str, mut customer: Customer) -> Result<Customer, SupportError> {
        if !self.customers.contains_key(customer_id) {
            return Err(SupportError::CustomerNotFound(customer_id.to_string()));
        }
        customer.id = customer_id.to_string();
        customer.updated_at = chrono::Utc::now();
        customer.calculate_segment();
        let _old = self.customers.insert(customer_id.to_string(), customer.clone());
        Ok(customer)
    }

    pub fn delete_customer(&mut self, customer_id: &str) -> Result<(), SupportError> {
        if self.customers.remove(customer_id).is_none() {
            return Err(SupportError::CustomerNotFound(customer_id.to_string()));
        }
        Ok(())
    }

    // Ticket CRUD
    pub fn create_ticket(&mut self, mut ticket: Ticket) -> Result<Ticket, SupportError> {
        self.ticket_counter += 1;
        
        if self.config.auto_assign {
            ticket = self.auto_assign(ticket)?;
        }
        
        let sla_tier = match ticket.priority {
            TicketPriority::Critical => &SlaTier::Vip,
            TicketPriority::High => &SlaTier::Express,
            TicketPriority::Normal => &SlaTier::Normal,
            TicketPriority::Low => &SlaTier::Economy,
        };
        
        for rule in &self.config.escalation_rules {
            if rule.trigger_keywords.iter().any(|kw| ticket.subject.to_lowercase().contains(kw)) {
                if let Some(ref sla) = rule.sla_override {
                    ticket.sla_due = Some(sla.next_deadline());
                    break;
                }
            }
        }
        
        if ticket.sla_due.is_none() {
            ticket.sla_due = Some(sla_tier.next_deadline());
        }
        
        if ticket.assignees.is_empty() {
            for rule in &self.config.escalation_rules {
                if rule.trigger_keywords.iter().any(|kw| ticket.subject.to_lowercase().contains(kw)) {
                    if let Some(ref assignee) = rule.assign_to {
                        ticket.assignees.push(assignee.clone());
                    }
                    break;
                }
            }
        }
        
        let ticket_id = ticket.id.clone();
        self.tickets.insert(ticket_id, ticket.clone());
        
        // Update customer stats
        if let Some(customer) = self.customers.get_mut(&ticket.customer_id) {
            customer.stats.tickets_created += 1;
        }
        
        Ok(ticket)
    }

    pub fn get_ticket(&self, ticket_id: &str) -> Option<&Ticket> {
        self.tickets.get(ticket_id)
    }

    pub fn get_ticket_mut(&mut self, ticket_id: &str) -> Option<&mut Ticket> {
        self.tickets.get_mut(ticket_id)
    }

    pub fn list_tickets(&self) -> Vec<&Ticket> {
        self.tickets.values().collect()
    }

    pub fn list_tickets_by_status(&self, status: &TicketStatus) -> Vec<&Ticket> {
        self.tickets.values()
            .filter(|t| &t.status == status)
            .collect()
    }

    pub fn list_tickets_by_customer(&self, customer_id: &str) -> Vec<&Ticket> {
        self.tickets.values()
            .filter(|t| t.customer_id == customer_id)
            .collect()
    }

    pub fn update_ticket(&mut self, ticket_id: &str, mut ticket: Ticket) -> Result<Ticket, SupportError> {
        if !self.tickets.contains_key(ticket_id) {
            return Err(SupportError::TicketNotFound(ticket_id.to_string()));
        }
        ticket.id = ticket_id.to_string();
        let _old = self.tickets.insert(ticket_id.to_string(), ticket.clone());
        Ok(ticket)
    }

    pub fn delete_ticket(&mut self, ticket_id: &str) -> Result<Ticket, SupportError> {
        self.tickets.remove(ticket_id)
            .ok_or_else(|| SupportError::TicketNotFound(ticket_id.to_string()))
    }

    pub fn add_message(&mut self, ticket_id: &str, sender_id: String, sender_name: String, content: String) -> Result<TicketMessage, SupportError> {
        // Calculate sentiment first (before mutable borrow)
        let sentiment = self.calculate_sentiment(&content);
        
        let ticket = self.tickets.get_mut(ticket_id)
            .ok_or_else(|| SupportError::TicketNotFound(ticket_id.to_string()))?;
        
        let message = TicketMessage::new(ticket_id, sender_id, sender_name, content);
        ticket.messages.push(message.clone());
        
        // Update priority based on sentiment
        if sentiment < self.config.sentiment_threshold {
            ticket.priority = TicketPriority::High;
        }
        
        Ok(message)
    }

    pub fn change_status(&mut self, ticket_id: &str, new_status: TicketStatus) -> Result<Ticket, SupportError> {
        let ticket = self.tickets.get_mut(ticket_id)
            .ok_or_else(|| SupportError::TicketNotFound(ticket_id.to_string()))?;
        
        // Validate transition
        if !Self::is_valid_transition(&ticket.status, &new_status) {
            return Err(SupportError::InvalidTransition(
                format!("{:?} -> {:?}", ticket.status, new_status)
            ));
        }
        
        let customer_id = ticket.customer_id.clone();
        let _old_status = std::mem::replace(&mut ticket.status, new_status.clone());
        
        // Update customer stats if resolved
        if new_status == TicketStatus::Resolved || new_status == TicketStatus::Closed {
            if let Some(customer) = self.customers.get_mut(&customer_id) {
                customer.stats.tickets_resolved += 1;
            }
        }
        
        Ok(ticket.clone())
    }

    pub fn assign_ticket(&mut self, ticket_id: &str, assignee: String) -> Result<Ticket, SupportError> {
        let ticket = self.tickets.get_mut(ticket_id)
            .ok_or_else(|| SupportError::TicketNotFound(ticket_id.to_string()))?;
        
        if !ticket.assignees.contains(&assignee) {
            ticket.assignees.push(assignee);
        }
        
        Ok(ticket.clone())
    }

    pub fn escalate_ticket(&mut self, ticket_id: &str) -> Result<Ticket, SupportError> {
        let ticket = self.tickets.get_mut(ticket_id)
            .ok_or_else(|| SupportError::TicketNotFound(ticket_id.to_string()))?;
        
        ticket.status = TicketStatus::Escalated;
        ticket.priority = TicketPriority::Critical;
        
        Ok(ticket.clone())
    }

    pub fn resolve_ticket(&mut self, ticket_id: &str) -> Result<Ticket, SupportError> {
        let ticket = self.tickets.get_mut(ticket_id)
            .ok_or_else(|| SupportError::TicketNotFound(ticket_id.to_string()))?;
        
        ticket.status = TicketStatus::Resolved;
        
        // Update customer
        if let Some(customer) = self.customers.get_mut(&ticket.customer_id) {
            customer.stats.tickets_resolved += 1;
        }
        
        Ok(ticket.clone())
    }

    pub fn close_ticket(&mut self, ticket_id: &str) -> Result<Ticket, SupportError> {
        let ticket = self.tickets.get_mut(ticket_id)
            .ok_or_else(|| SupportError::TicketNotFound(ticket_id.to_string()))?;
        
        ticket.status = TicketStatus::Closed;
        Ok(ticket.clone())
    }

    fn is_valid_transition(from: &TicketStatus, to: &TicketStatus) -> bool {
        match (from, to) {
            (TicketStatus::Open, TicketStatus::Pending) => true,
            (TicketStatus::Open, TicketStatus::InProgress) => true,
            (TicketStatus::Open, TicketStatus::Escalated) => true,
            (TicketStatus::Open, TicketStatus::Closed) => true,
            (TicketStatus::Pending, TicketStatus::InProgress) => true,
            (TicketStatus::Pending, TicketStatus::Waiting) => true,
            (TicketStatus::Pending, TicketStatus::Escalated) => true,
            (TicketStatus::InProgress, TicketStatus::Waiting) => true,
            (TicketStatus::InProgress, TicketStatus::Resolved) => true,
            (TicketStatus::InProgress, TicketStatus::Escalated) => true,
            (TicketStatus::Waiting, TicketStatus::InProgress) => true,
            (TicketStatus::Waiting, TicketStatus::Resolved) => true,
            (TicketStatus::Escalated, TicketStatus::InProgress) => true,
            (TicketStatus::Escalated, TicketStatus::Resolved) => true,
            (TicketStatus::Resolved, TicketStatus::Closed) => true,
            (TicketStatus::Resolved, TicketStatus::Open) => true, // Reopen
            (TicketStatus::Closed, TicketStatus::Open) => true, // Reopen
            _ => false,
        }
    }

    fn auto_assign(&self, mut ticket: Ticket) -> Result<Ticket, SupportError> {
        let search_text = ticket.subject.to_lowercase();
        
        for rule in &self.config.escalation_rules {
            for keyword in &rule.trigger_keywords {
                if search_text.contains(keyword) {
                    ticket.priority = rule.priority.clone();
                    if let Some(ref assignee) = rule.assign_to {
                        ticket.assignees.push(assignee.clone());
                    }
                    break;
                }
            }
        }
        
        Ok(ticket)
    }
    
    pub fn calculate_sentiment(&self, text: &str) -> f32 {
        let lower = text.to_lowercase();
        
        let positive = ["cảm ơn", "tốt", "hữu ích", "nhanh", "uy tín", "chuyên nghiệp", "hài lòng", "tuyệt vời", "wow", "amazing", "great", "spasibo", "merci", "excellent"];
        let negative = ["chậm", "lỗi", "sai", "hỏng", "kinh tởm", "tệ", "không hài lòng", "hoàn tiền", "hủy đơn", "khiếu nại", "tồi tệ", "disappointed"];
        let urgent = ["gấp", "khẩn", "immediately", "ngay", "bây giờ", "trước 12h", "khách hàng VIP", "đơn lớn", "emergency", "urgent"];
        
        let pos_count = positive.iter().filter(|w| lower.contains(*w)).count() as f32;
        let neg_count = negative.iter().filter(|w| lower.contains(*w)).count() as f32;
        let urgent_count = urgent.iter().filter(|w| lower.contains(*w)).count() as f32;
        
        let sentiment = (pos_count - neg_count + urgent_count) / (pos_count + neg_count + 1.0);
        sentiment.max(-1.0).min(1.0)
    }
    
    pub fn should_escalate(&self, ticket: &Ticket) -> bool {
        if ticket.is_sla_violated() {
            return true;
        }
        matches!(ticket.priority, TicketPriority::Critical | TicketPriority::High)
    }

    pub fn get_sla_violations(&self) -> Vec<&Ticket> {
        self.tickets.values()
            .filter(|t| t.is_sla_violated() && !matches!(t.status, TicketStatus::Resolved | TicketStatus::Closed))
            .collect()
    }

    pub fn get_pending_tickets(&self) -> Vec<&Ticket> {
        self.tickets.values()
            .filter(|t| !matches!(t.status, TicketStatus::Resolved | TicketStatus::Closed))
            .collect()
    }

    pub fn export_tickets_json(&self) -> String {
        serde_json::to_string_pretty(&self.tickets).unwrap_or_default()
    }
}

impl Default for SupportAgent {
    fn default() -> Self {
        Self::with_default_config()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SupportError {
    #[error("Ticket not found: {0}")]
    TicketNotFound(String),
    #[error("Customer not found: {0}")]
    CustomerNotFound(String),
    #[error("Invalid transition: {0}")]
    InvalidTransition(String),
    #[error("SLA violation: {0}")]
    SlaViolation(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentiment_detection_positive() {
        let agent = SupportAgent::with_default_config();
        let score = agent.calculate_sentiment("Cảm ơn shop rất nhiều, giao hàng nhanh, nhân viên chuyên nghiệp");
        assert!(score > 0.0);
    }
    
    #[test]
    fn test_sentiment_detection_negative() {
        let agent = SupportAgent::with_default_config();
        let score = agent.calculate_sentiment("Giao hàng chậm quá, sản phẩm lỗi, không hài lòng");
        assert!(score < 0.0);
    }
    
    #[test]
    fn test_create_and_manage_ticket() {
        let mut agent = SupportAgent::with_default_config();
        
        // Create customer
        let customer = agent.create_customer("Nguyen Van A".to_string());
        assert!(!customer.id.is_empty());
        
        // Create ticket
        let ticket = Ticket::new(
            customer.id.clone(),
            TicketChannel::Zalo,
            "Tôi muốn hủy đơn hàng này ngay".to_string(),
        );
        
        let result = agent.create_ticket(ticket).unwrap();
        assert!(matches!(result.priority, TicketPriority::Critical));
        assert!(!result.assignees.is_empty()); // Should be auto-assigned
        
        // Add message
        let msg = agent.add_message(&result.id, "customer_1".to_string(), "Nguyen Van A".to_string(), "Xin chào".to_string()).unwrap();
        assert!(!msg.id.is_empty());
        
        // Change status
        let updated = agent.change_status(&result.id, TicketStatus::InProgress).unwrap();
        assert!(matches!(updated.status, TicketStatus::InProgress));
        
        // Resolve ticket
        let resolved = agent.resolve_ticket(&result.id).unwrap();
        assert!(matches!(resolved.status, TicketStatus::Resolved));
    }

    #[test]
    fn test_invalid_transition() {
        let mut agent = SupportAgent::with_default_config();
        let customer = agent.create_customer("Test".to_string());
        let ticket = agent.create_ticket(Ticket::new(customer.id, TicketChannel::Zalo, "Test".to_string())).unwrap();
        
        // Can't go from Open to Resolved directly
        let result = agent.change_status(&ticket.id, TicketStatus::Resolved);
        assert!(result.is_err());
    }

    #[test]
    fn test_customer_segment_update() {
        let mut agent = SupportAgent::with_default_config();
        let mut customer = agent.create_customer("VIP Customer".to_string());
        
        customer.stats.total_spent = 15_000_000;
        agent.update_customer(&customer.id, customer.clone()).unwrap();
        
        let updated = agent.get_customer(&customer.id).unwrap();
        assert!(matches!(updated.segment, CustomerSegment::VIP));
    }

    #[test]
    fn test_sla_violation_tracking() {
        let mut agent = SupportAgent::with_default_config();
        let customer = agent.create_customer("Test".to_string());
        
        let mut ticket = Ticket::new(customer.id, TicketChannel::Zalo, "Test".to_string());
        ticket.sla_due = Some(chrono::Utc::now() - chrono::Duration::hours(2));
        
        let result = agent.create_ticket(ticket).unwrap();
        let violations = agent.get_sla_violations();
        
        assert!(!violations.is_empty());
        assert_eq!(violations[0].id, result.id);
    }
}
