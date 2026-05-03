//! BizClaw Accounting Agent - Bookkeeping for Vietnamese OPC
//! 
//! ## Features
//! - Double-entry bookkeeping ledger
//! - Vietnamese tax calculations (VAT, PIT, CIT)
//! - Financial reports (Balance Sheet, Income Statement, Cash Flow)
//! - Multi-currency support (VND)
//! - FPT Eyesoft compatible exports

pub mod transaction;
pub mod ledger;
pub mod tax;
pub mod report;

pub use transaction::{Account, AccountType, Transaction, TransactionEntry, TransactionType};
pub use ledger::Ledger;
pub use tax::{TaxCalculator, VatReport};
pub use report::{FinancialReport, ReportType, BalanceSheet, IncomeStatement, CashFlow, TrialBalance, TrialBalanceEntry};

pub struct AccountingAgent {
    pub ledger: Ledger,
    pub tax_calculator: TaxCalculator,
}

impl AccountingAgent {
    pub fn new() -> Self {
        Self {
            ledger: Ledger::new("Main Ledger"),
            tax_calculator: TaxCalculator::vietnam_standard(),
        }
    }

    pub fn record_sale(&mut self, amount: i64, customer_id: &str) -> anyhow::Result<Transaction> {
        let vat = self.tax_calculator.calculate_vat(amount, false);
        let total = amount + vat;
        
        self.ledger.add_transaction(
            &format!("Sale to customer {}", customer_id),
            vec![
                ("1200".to_string(), total, true),
                ("5000".to_string(), amount, false),
                ("2200".to_string(), vat, false),
            ],
        )
    }

    pub fn record_purchase(&mut self, amount: i64, vendor_id: &str) -> anyhow::Result<Transaction> {
        let vat = self.tax_calculator.calculate_vat(amount, false);
        let total = amount + vat;
        
        self.ledger.add_transaction(
            &format!("Purchase from vendor {}", vendor_id),
            vec![
                ("1500".to_string(), amount, true),
                ("2200".to_string(), vat, true),
                ("2100".to_string(), total, false),
            ],
        )
    }

    pub fn record_expense(&mut self, amount: i64, category: &str) -> anyhow::Result<Transaction> {
        let account_code = match category {
            "selling" => "6100",
            "admin" => "6200",
            _ => "6200",
        };
        
        self.ledger.add_transaction(
            &format!("Expense: {}", category),
            vec![
                (account_code.to_string(), amount, true),
                ("1100".to_string(), amount, false),
            ],
        )
    }

    pub fn generate_balance_sheet(&self) -> BalanceSheet {
        BalanceSheet {
            date: chrono::Utc::now(),
            assets: self.ledger.total_assets(),
            liabilities: self.ledger.total_liabilities(),
            equity: self.ledger.total_equity(),
        }
    }

    pub fn generate_income_statement(&self) -> IncomeStatement {
        let revenue = self.ledger.get_account_balance("5000");
        let cogs = self.ledger.get_account_balance("6000");
        let opex = self.ledger.get_account_balance("6100") + self.ledger.get_account_balance("6200");
        
        IncomeStatement::calculate(revenue, cogs, opex)
    }

    pub fn generate_trial_balance(&self) -> TrialBalance {
        let mut accounts: Vec<TrialBalanceEntry> = Vec::new();
        
        for account in self.ledger.accounts.values() {
            let balance = account.balance();
            if balance != 0 {
                accounts.push(TrialBalanceEntry {
                    account_code: account.code.clone(),
                    account_name: account.name.clone(),
                    debit: if balance > 0 && matches!(account.account_type, AccountType::Asset | AccountType::Expense) { balance } else { 0 },
                    credit: if balance < 0 || matches!(account.account_type, AccountType::Liability | AccountType::Equity | AccountType::Revenue) { balance.abs() } else { 0 },
                });
            }
        }
        
        let total_debit: i64 = accounts.iter().map(|a| a.debit).sum();
        let total_credit: i64 = accounts.iter().map(|a| a.credit).sum();
        
        TrialBalance {
            date: chrono::Utc::now(),
            accounts,
            total_debit,
            total_credit,
            balanced: total_debit == total_credit,
        }
    }

    pub fn calculate_cash_flow(&self) -> CashFlow {
        let cash_accounts = ["1000", "1100"];
        let operating_cash: i64 = cash_accounts.iter()
            .filter_map(|code| self.ledger.accounts.get(*code))
            .map(|a| a.balance())
            .sum();
        
        CashFlow {
            period: chrono::Utc::now().format("%Y-%m").to_string(),
            operating: operating_cash,
            investing: 0,
            financing: 0,
            net_cash: operating_cash,
        }
    }

    pub fn record_payment_received(&mut self, from_customer: &str, amount: i64) -> anyhow::Result<Transaction> {
        self.ledger.add_transaction(
            &format!("Payment received from {}", from_customer),
            vec![
                ("1100".to_string(), amount, true),
                ("1200".to_string(), amount, false),
            ],
        )
    }

    pub fn record_payment_made(&mut self, to_vendor: &str, amount: i64) -> anyhow::Result<Transaction> {
        self.ledger.add_transaction(
            &format!("Payment made to {}", to_vendor),
            vec![
                ("2100".to_string(), amount, true),
                ("1100".to_string(), amount, false),
            ],
        )
    }

    pub fn generate_vat_report(&self, period_start: chrono::NaiveDate, period_end: chrono::NaiveDate) -> VatReport {
        let sales: i64 = self.ledger.transactions.iter()
            .filter(|tx| {
                tx.created_at.date_naive() >= period_start && tx.created_at.date_naive() <= period_end
            })
            .filter(|tx| tx.description.contains("Sale"))
            .map(|tx| tx.entries.iter().filter(|(code, _, is_debit)| *code == "2200" && !*is_debit).map(|(_, amt, _)| amt).sum::<i64>())
            .sum();
        
        VatReport::quarterly(sales, 0, 10.0)
    }

    pub fn export_to_json(&self) -> String {
        serde_json::to_string_pretty(&self.ledger).unwrap_or_default()
    }
}

impl Default for AccountingAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_entry_balance() {
        let mut ledger = Ledger::new("Test");
        let result = ledger.add_transaction(
            "Test sale",
            vec![
                ("1200".to_string(), 100_000, true),
                ("5000".to_string(), 100_000, false),
            ],
        );
        assert!(result.is_ok());
        assert_eq!(ledger.get_account_balance("1200"), 100_000);
        assert_eq!(ledger.get_account_balance("5000"), 100_000);
    }

    #[test]
    fn test_unbalanced_transaction_fails() {
        let mut ledger = Ledger::new("Test");
        let result = ledger.add_transaction(
            "Unbalanced",
            vec![
                ("1200".to_string(), 100_000, true),
                ("5000".to_string(), 50_000, false),
            ],
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_vat_calculation() {
        let calc = TaxCalculator::vietnam_standard();
        assert_eq!(calc.calculate_vat(1_000_000, false), 100_000);
        assert_eq!(calc.calculate_vat(1_100_000, true), 100_000);
    }

    #[test]
    fn test_accounting_agent_sale() {
        let mut agent = AccountingAgent::new();
        let result = agent.record_sale(1_000_000, "CUST001");
        assert!(result.is_ok());
        
        let bs = agent.generate_balance_sheet();
        assert!(bs.assets > 0);
    }
}
