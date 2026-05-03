//! BizClaw Inventory Agent - Stock management for Vietnamese OPC
//! 
//! ## Features
//! - Multi-warehouse inventory tracking
//! - Real-time stock levels
//! - Low stock alerts (Zalo/Email)
//! - Batch tracking (Hạn sử dụng)
//! - Stock valuation (FIFO/Average)
//! - Integration with POS and Accounting

pub mod stock;
pub mod warehouse;
pub mod alert;

pub use stock::{InventoryItem, StockTransaction};
pub use warehouse::Warehouse;
pub use alert::{Alert, AlertType, AlertSeverity, StockAlert};

pub struct InventoryAgent {
    pub items: std::collections::HashMap<String, InventoryItem>,
    pub warehouses: std::collections::HashMap<String, Warehouse>,
    pub transactions: Vec<StockTransaction>,
}

impl InventoryAgent {
    pub fn new() -> Self {
        let mut agent = Self {
            items: std::collections::HashMap::new(),
            warehouses: std::collections::HashMap::new(),
            transactions: Vec::new(),
        };
        
        let main_wh = Warehouse::default_store("Cửa hàng chính", "TP.HCM");
        agent.warehouses.insert(main_wh.id.clone(), main_wh);
        
        agent
    }

    pub fn add_warehouse(&mut self, code: &str, name: &str, address: &str) -> Warehouse {
        let wh = Warehouse::new(code, name, address);
        self.warehouses.insert(wh.id.clone(), wh.clone());
        wh
    }

    pub fn add_item(&mut self, sku: &str, name: &str, cost: i64) -> InventoryItem {
        let default_wh = self.warehouses.values()
            .find(|w| w.is_default)
            .map(|w| w.id.clone())
            .unwrap_or_default();
        
        let item = InventoryItem::new(sku, name, &default_wh).with_stock(0, cost);
        self.items.insert(item.id.clone(), item.clone());
        item
    }

    pub fn receive_stock(&mut self, item_id: &str, qty: i32, reason: &str) -> Option<StockTransaction> {
        if let Some(item) = self.items.get_mut(item_id) {
            let tx = item.adjust(qty, reason);
            self.transactions.push(tx.clone());
            Some(tx)
        } else {
            None
        }
    }

    pub fn sell_stock(&mut self, item_id: &str, qty: i32) -> Option<StockTransaction> {
        if let Some(item) = self.items.get_mut(item_id) {
            if item.available >= qty {
                let tx = item.adjust(-qty, "Bán hàng");
                self.transactions.push(tx.clone());
                Some(tx)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn reserve_stock(&mut self, item_id: &str, qty: i32) -> bool {
        if let Some(item) = self.items.get_mut(item_id) {
            item.reserve(qty)
        } else {
            false
        }
    }

    pub fn release_reserved(&mut self, item_id: &str, qty: i32) -> bool {
        if let Some(item) = self.items.get_mut(item_id) {
            if item.reserved >= qty {
                item.reserved -= qty;
                item.available += qty;
                item.updated_at = chrono::Utc::now();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn fulfill_reserved(&mut self, item_id: &str, qty: i32) -> bool {
        if let Some(item) = self.items.get_mut(item_id) {
            if item.reserved >= qty {
                item.reserved -= qty;
                item.quantity -= qty;
                item.available = item.quantity - item.reserved;
                item.updated_at = chrono::Utc::now();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn check_low_stock(&self) -> Vec<StockAlert> {
        self.items.values()
            .filter(|item| item.is_low_stock())
            .map(|item| {
                let alert_type = if item.available == 0 {
                    AlertType::OutOfStock
                } else {
                    AlertType::LowStock
                };
                StockAlert {
                    item_id: item.id.clone(),
                    item_name: item.name.clone(),
                    current_qty: item.available,
                    reorder_point: item.reorder_point,
                    alert_type,
                }
            })
            .collect()
    }

    pub fn get_item(&self, item_id: &str) -> Option<&InventoryItem> {
        self.items.get(item_id)
    }

    pub fn get_item_by_sku(&self, sku: &str) -> Option<&InventoryItem> {
        self.items.values().find(|i| i.sku == sku)
    }

    pub fn total_inventory_value(&self) -> i64 {
        self.items.values()
            .map(|item| item.quantity as i64 * item.cost)
            .sum()
    }

    pub fn export_inventory_report(&self) -> serde_json::Value {
        let items: Vec<_> = self.items.values()
            .map(|item| {
                serde_json::json!({
                    "id": item.id,
                    "sku": item.sku,
                    "name": item.name,
                    "category": item.category,
                    "quantity": item.quantity,
                    "available": item.available,
                    "reserved": item.reserved,
                    "cost": item.cost,
                    "value": item.quantity as i64 * item.cost,
                    "low_stock": item.is_low_stock()
                })
            })
            .collect();
        
        serde_json::json!({
            "total_items": self.items.len(),
            "total_value": self.total_inventory_value(),
            "low_stock_count": self.check_low_stock().len(),
            "items": items
        })
    }
}

impl Default for InventoryAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_item() {
        let mut agent = InventoryAgent::new();
        let item = agent.add_item("SKU001", "Sản phẩm A", 100_000);
        
        assert_eq!(item.sku, "SKU001");
        assert_eq!(item.quantity, 0);
    }

    #[test]
    fn test_receive_stock() {
        let mut agent = InventoryAgent::new();
        let item = agent.add_item("SKU001", "Sản phẩm A", 100_000);
        
        let tx = agent.receive_stock(&item.id, 100, "Nhập hàng mới");
        assert!(tx.is_some());
        
        let updated = agent.get_item(&item.id).unwrap();
        assert_eq!(updated.quantity, 100);
    }

    #[test]
    fn test_sell_stock() {
        let mut agent = InventoryAgent::new();
        let item = agent.add_item("SKU001", "Sản phẩm A", 100_000);
        
        agent.receive_stock(&item.id, 100, "Nhập hàng");
        let tx = agent.sell_stock(&item.id, 10);
        assert!(tx.is_some());
        
        let updated = agent.get_item(&item.id).unwrap();
        assert_eq!(updated.quantity, 90);
    }

    #[test]
    fn test_low_stock_alert() {
        let mut agent = InventoryAgent::new();
        let item = agent.add_item("SKU001", "Sản phẩm A", 100_000);
        
        let item_id = item.id.clone();
        if let Some(item) = agent.items.get_mut(&item_id) {
            item.reorder_point = 20;
            item.quantity = 15;
            item.available = 15;
        }
        
        let alerts = agent.check_low_stock();
        assert!(!alerts.is_empty());
    }

    #[test]
    fn test_reserve_and_sell() {
        let mut agent = InventoryAgent::new();
        let item = agent.add_item("SKU001", "Sản phẩm A", 100_000);
        
        agent.receive_stock(&item.id, 100, "Nhập hàng");
        
        let reserved = agent.reserve_stock(&item.id, 30);
        assert!(reserved);
        
        let item = agent.get_item(&item.id).unwrap();
        assert_eq!(item.available, 70);
        assert_eq!(item.reserved, 30);
    }

    #[test]
    fn test_release_reserved() {
        let mut agent = InventoryAgent::new();
        let item = agent.add_item("SKU001", "Sản phẩm A", 100_000);
        
        agent.receive_stock(&item.id, 100, "Nhập hàng");
        agent.reserve_stock(&item.id, 30);
        
        let released = agent.release_reserved(&item.id, 20);
        assert!(released);
        
        let item = agent.get_item(&item.id).unwrap();
        assert_eq!(item.reserved, 10);
        assert_eq!(item.available, 90);
    }

    #[test]
    fn test_fulfill_reserved() {
        let mut agent = InventoryAgent::new();
        let item = agent.add_item("SKU001", "Sản phẩm A", 100_000);
        
        agent.receive_stock(&item.id, 100, "Nhập hàng");
        agent.reserve_stock(&item.id, 30);
        
        let fulfilled = agent.fulfill_reserved(&item.id, 30);
        assert!(fulfilled);
        
        let item = agent.get_item(&item.id).unwrap();
        assert_eq!(item.reserved, 0);
        assert_eq!(item.quantity, 70);
        assert_eq!(item.available, 70);
    }

    #[test]
    fn test_get_all_items() {
        let mut agent = InventoryAgent::new();
        agent.add_item("SKU001", "Sản phẩm A", 100_000);
        agent.add_item("SKU002", "Sản phẩm B", 200_000);
        
        let items: Vec<_> = agent.items.values().collect();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_warehouse_management() {
        let mut agent = InventoryAgent::new();
        
        let wh = agent.add_warehouse("WH001", "Kho HCM", "Quận 1, HCM");
        assert_eq!(wh.code, "WH001");
        
        assert_eq!(agent.warehouses.len(), 2); // Default + new
    }
}
