//! BizClaw POS Agent - Point of Sale for Vietnamese OPC
//! 
//! ## Features
//! - Barcode/QR scanning support
//! - VietQR payment integration
//! - Multi-payment (Cash, MoMo, VietQR, ZaloPay)
//! - Receipt printing (thermal printer format)
//! - Customer loyalty tracking
//! - Daily sales summary
//! - Integration with Inventory and Accounting

pub mod product;
pub mod sale;
pub mod receipt;

pub use product::{Product, ProductCatalog};
pub use sale::{Sale, SaleItem, SaleStatus};
pub use receipt::Receipt;

pub struct POSAgent {
    pub catalog: ProductCatalog,
    pub store_name: String,
    pub store_address: String,
    pub store_phone: String,
    pub vat_rate: f32,
    pub sales: Vec<Sale>,
}

impl POSAgent {
    pub fn new(store_name: &str, store_address: &str, store_phone: &str) -> Self {
        Self {
            catalog: ProductCatalog::new(),
            store_name: store_name.to_string(),
            store_address: store_address.to_string(),
            store_phone: store_phone.to_string(),
            vat_rate: 10.0,
            sales: Vec::new(),
        }
    }

    pub fn add_product(&mut self, sku: &str, name: &str, price: i64) -> Product {
        let product = Product::new(sku, name, price);
        self.catalog.add_product(product.clone());
        product
    }

    pub fn get_product(&self, product_id: &str) -> Option<&Product> {
        self.catalog.get(product_id)
    }

    pub fn get_product_by_sku(&self, sku: &str) -> Option<&Product> {
        self.catalog.find_by_sku(sku)
    }

    pub fn get_product_by_barcode(&self, barcode: &str) -> Option<&Product> {
        self.catalog.find_by_barcode(barcode)
    }

    pub fn get_all_products(&self) -> Vec<&Product> {
        self.catalog.get_all()
    }

    pub fn get_products_by_category(&self, category: &str) -> Vec<&Product> {
        self.catalog.get_by_category(category)
    }

    pub fn update_product(&mut self, product_id: &str, mut product: Product) -> Option<Product> {
        product.id = product_id.to_string();
        self.catalog.get_mut(product_id);
        if let Some(existing) = self.catalog.get_mut(product_id) {
            existing.name = product.name.clone();
            existing.price = product.price;
            existing.category = product.category.clone();
            existing.barcode = product.barcode.clone();
            existing.unit = product.unit.clone();
            Some(existing.clone())
        } else {
            None
        }
    }

    pub fn deactivate_product(&mut self, product_id: &str) -> bool {
        if let Some(product) = self.catalog.get_mut(product_id) {
            product.active = false;
            true
        } else {
            false
        }
    }

    pub fn activate_product(&mut self, product_id: &str) -> bool {
        if let Some(product) = self.catalog.get_mut(product_id) {
            product.active = true;
            true
        } else {
            false
        }
    }

    pub fn delete_product(&mut self, product_id: &str) -> bool {
        self.catalog.remove(product_id).is_some()
    }

    pub fn create_sale(&self, cashier_id: &str) -> Sale {
        Sale::new(cashier_id)
    }

    pub fn add_item_to_sale(&self, sale: &mut Sale, product: &Product, quantity: i32) -> Option<SaleItem> {
        if product.stock_qty >= quantity && product.active {
            Some(SaleItem::new(product, quantity, 0))
        } else {
            None
        }
    }

    pub fn checkout(&mut self, mut sale: Sale, payment_method: &str) -> anyhow::Result<(Sale, Receipt)> {
        sale.payment_method = payment_method.to_string();
        sale.status = SaleStatus::Completed;
        
        let receipt = Receipt::new(
            &sale,
            &self.store_name,
            &self.store_address,
            &self.store_phone,
            &sale.cashier_id,
        );
        
        self.sales.push(sale.clone());
        
        Ok((sale, receipt))
    }

    pub fn calculate_change(&self, total: i64, amount_paid: i64) -> i64 {
        amount_paid.saturating_sub(total)
    }

    pub fn daily_summary(&self) -> DailySummary {
        let today = chrono::Utc::now().date_naive();
        let today_sales: Vec<_> = self.sales.iter()
            .filter(|s| s.created_at.date_naive() == today && s.status == SaleStatus::Completed)
            .collect();
        
        let total_revenue: i64 = today_sales.iter().map(|s| s.total).sum();
        let total_items: i32 = today_sales.iter()
            .flat_map(|s| s.items.iter())
            .map(|i| i.quantity)
            .sum();
        let transaction_count = today_sales.len() as u32;
        
        DailySummary {
            date: today,
            transaction_count,
            total_revenue,
            total_items,
            average_basket: if transaction_count > 0 { total_revenue / transaction_count as i64 } else { 0 },
        }
    }

    pub fn get_sales_by_date(&self, date: chrono::NaiveDate) -> Vec<&Sale> {
        self.sales.iter()
            .filter(|s| s.created_at.date_naive() == date && s.status == SaleStatus::Completed)
            .collect()
    }

    pub fn void_sale(&mut self, sale_id: &str) -> bool {
        if let Some(sale) = self.sales.iter_mut().find(|s| s.id == sale_id) {
            if sale.status == SaleStatus::Completed {
                sale.status = SaleStatus::Cancelled;
                return true;
            }
        }
        false
    }

    pub fn process_refund(&mut self, sale_id: &str) -> Option<Sale> {
        if let Some(idx) = self.sales.iter().position(|s| s.id == sale_id && s.status == SaleStatus::Completed) {
            self.sales[idx].status = SaleStatus::Refunded;
            Some(self.sales[idx].clone())
        } else {
            None
        }
    }

    pub fn generate_invoice(&self, sale: &Sale) -> String {
        let receipt = Receipt::new(
            sale,
            &self.store_name,
            &self.store_address,
            &self.store_phone,
            &sale.cashier_id,
        );
        receipt.format()
    }

    pub fn export_sales_report(&self, start_date: chrono::NaiveDate, end_date: chrono::NaiveDate) -> serde_json::Value {
        let filtered: Vec<_> = self.sales.iter()
            .filter(|s| {
                let date = s.created_at.date_naive();
                date >= start_date && date <= end_date && s.status == SaleStatus::Completed
            })
            .collect();
        
        let total_revenue: i64 = filtered.iter().map(|s| s.total).sum();
        
        serde_json::json!({
            "period": {
                "start": start_date.to_string(),
                "end": end_date.to_string()
            },
            "total_transactions": filtered.len(),
            "total_revenue": total_revenue,
            "average_basket": if !filtered.is_empty() { total_revenue / filtered.len() as i64 } else { 0 },
            "sales": filtered
        })
    }
}

impl Default for POSAgent {
    fn default() -> Self {
        Self::new("Cửa hàng BizClaw", "TP.HCM", "0909123456")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DailySummary {
    pub date: chrono::NaiveDate,
    pub transaction_count: u32,
    pub total_revenue: i64,
    pub total_items: i32,
    pub average_basket: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sale() {
        let pos = POSAgent::default();
        let sale = pos.create_sale("cashier_1");
        assert_eq!(sale.status, SaleStatus::Pending);
    }

    #[test]
    fn test_add_product() {
        let mut pos = POSAgent::default();
        let product = pos.add_product("SKU001", "Cà phê sữa", 35000);
        assert_eq!(product.price, 35000);
        
        let found = pos.catalog.find_by_sku("SKU001");
        assert!(found.is_some());
    }

    #[test]
    fn test_checkout() {
        let mut pos = POSAgent::default();
        let product = pos.add_product("SKU001", "Cà phê sữa", 35000);
        
        let mut sale = pos.create_sale("cashier_1");
        if let Some(item) = pos.add_item_to_sale(&mut sale, &product, 2) {
            sale.add_item(item);
        }
        
        sale.payment_method = "cash".to_string();
        sale.status = SaleStatus::Completed;
        
        let (completed, _receipt) = pos.checkout(sale, "cash").unwrap();
        assert_eq!(completed.status, SaleStatus::Completed);
    }

    #[test]
    fn test_change_calculation() {
        let pos = POSAgent::default();
        assert_eq!(pos.calculate_change(50000, 100000), 50000);
        assert_eq!(pos.calculate_change(100000, 50000), -50000);
    }

    #[test]
    fn test_void_sale() {
        let mut pos = POSAgent::default();
        let product = pos.add_product("SKU001", "Cà phê", 30000);
        
        let mut sale = pos.create_sale("cashier_1");
        if let Some(item) = pos.add_item_to_sale(&mut sale, &product, 1) {
            sale.add_item(item);
        }
        sale.status = SaleStatus::Completed;
        pos.sales.push(sale.clone());
        
        let result = pos.void_sale(&sale.id);
        assert!(result);
        
        let voided = pos.sales.iter().find(|s| s.id == sale.id).unwrap();
        assert_eq!(voided.status, SaleStatus::Cancelled);
    }

    #[test]
    fn test_product_crud() {
        let mut pos = POSAgent::default();
        let product = pos.add_product("SKU001", "Cà phê sữa", 35000);
        
        // Get product
        let found = pos.get_product(&product.id);
        assert!(found.is_some());
        
        // Get all products
        let all = pos.get_all_products();
        assert_eq!(all.len(), 1);
        
        // Deactivate
        let deactivated = pos.deactivate_product(&product.id);
        assert!(deactivated);
        
        // Activate
        let activated = pos.activate_product(&product.id);
        assert!(activated);
        
        // Delete
        let deleted = pos.delete_product(&product.id);
        assert!(deleted);
        assert_eq!(pos.get_all_products().len(), 0);
    }

    #[test]
    fn test_products_by_category() {
        let mut pos = POSAgent::default();
        let p1 = pos.add_product("SKU001", "Cà phê", 35000);
        let p2 = pos.add_product("SKU002", "Trà sữa", 40000);
        
        // Update category for p1
        pos.update_product(&p1.id, Product {
            category: "Thức uống".to_string(),
            ..p1.clone()
        });
        
        let drinks = pos.get_products_by_category("Thức uống");
        assert_eq!(drinks.len(), 1);
    }

    #[test]
    fn test_search_products() {
        let mut pos = POSAgent::default();
        pos.add_product("SKU001", "Cà phê sữa đá", 35000);
        pos.add_product("SKU002", "Trà sữa", 40000);
        
        let results = pos.catalog.search("cà phê");
        assert_eq!(results.len(), 1);
    }
}
