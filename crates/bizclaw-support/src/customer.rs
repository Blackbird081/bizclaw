//! Customer 360 profile for Vietnamese customers

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub zalo_id: Option<String>,
    pub facebook_id: Option<String>,
    pub telegram_id: Option<String>,
    pub segment: CustomerSegment,
    pub stats: CustomerStats,
    pub tags: Vec<String>,
    pub metadata: std::collections::HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustomerSegment {
    VIP,
    Regular,
    New,
    AtRisk,
    Churned,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomerStats {
    pub total_orders: u32,
    pub total_spent: i64,
    pub avg_order_value: f64,
    pub last_order_date: Option<DateTime<Utc>>,
    pub tickets_created: u32,
    pub tickets_resolved: u32,
    pub sentiment_score: f32,
}

impl Customer {
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            phone: None,
            email: None,
            zalo_id: None,
            facebook_id: None,
            telegram_id: None,
            segment: CustomerSegment::New,
            stats: CustomerStats::default(),
            tags: Vec::new(),
            metadata: std::collections::HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn with_zalo(mut self, zalo_id: &str) -> Self {
        self.zalo_id = Some(zalo_id.to_string());
        self
    }

    pub fn with_phone(mut self, phone: &str) -> Self {
        self.phone = Some(phone.to_string());
        self
    }

    pub fn with_email(mut self, email: &str) -> Self {
        self.email = Some(email.to_string());
        self
    }

    pub fn with_facebook(mut self, fb_id: &str) -> Self {
        self.facebook_id = Some(fb_id.to_string());
        self
    }

    pub fn with_telegram(mut self, tg_id: &str) -> Self {
        self.telegram_id = Some(tg_id.to_string());
        self
    }

    pub fn add_tag(&mut self, tag: &str) {
        if !self.tags.contains(&tag.to_string()) {
            self.tags.push(tag.to_string());
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    pub fn calculate_segment(&mut self) {
        if self.stats.total_spent > 10_000_000 {
            self.segment = CustomerSegment::VIP;
        } else if self.stats.sentiment_score < -0.5 || self.stats.last_order_date.map(|d| (Utc::now() - d).num_days() > 90).unwrap_or(false) {
            self.segment = CustomerSegment::AtRisk;
        } else if self.stats.total_orders == 0 {
            self.segment = CustomerSegment::New;
        } else if self.resolution_rate() < 0.8 {
            self.segment = CustomerSegment::AtRisk;
        } else {
            self.segment = CustomerSegment::Regular;
        }
        self.updated_at = Utc::now();
    }
    
    pub fn resolution_rate(&self) -> f32 {
        if self.stats.tickets_created == 0 {
            return 1.0;
        }
        self.stats.tickets_resolved as f32 / self.stats.tickets_created as f32
    }

    pub fn lifetime_value(&self) -> f64 {
        self.stats.total_spent as f64 * (1.0 + self.stats.sentiment_score as f64)
    }

    pub fn add_purchase(&mut self, amount: i64) {
        self.stats.total_orders += 1;
        self.stats.total_spent += amount;
        self.stats.avg_order_value = self.stats.total_spent as f64 / self.stats.total_orders as f64;
        self.stats.last_order_date = Some(Utc::now());
        self.calculate_segment();
    }
}
