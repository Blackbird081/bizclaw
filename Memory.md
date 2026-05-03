# BizClaw - Persistent Memory

> **Last Updated:** 2025-01-17  
> **Version:** 1.1.7  
> **Language:** Vietnamese OPC (One Person Company) ERP

---

## 🏢 TỔNG QUAN HỆ THỐNG

### Core Identity
**BizClaw** là hệ thống ERP agent-based cho **chủ OPC Việt Nam**, viết bằng **Rust** với kiến trúc multi-agent. Tên gọi "BizClaw" = Business + Claw (móng vuốt kinh doanh).

### Technology Stack
- **Language:** Rust (Tokio async runtime)
- **Architecture:** Multi-agent orchestration (giống Agency-Agents nhưng thực sự chạy được)
- **Modules:** 55+ crates trong workspace
- **Key Integrations:** VietQR, MoMo, ZaloPay, VietinBank, Zalo OA, Facebook Messenger, Telegram

---

## 📦 MODULES ĐÃ IMPLEMENT - 100% COMPLETE

### ✅ Core Modules (Production Ready)

| Module | Tests | Status | Features |
|--------|-------|--------|----------|
| **bizclaw-payment** | 3 + 1 doc | ✅ FULL | VietQR generation, CRC16 checksum, MoMo/ZaloPay API, format_vnd with separators |
| **bizclaw-support** | 6 | ✅ FULL | Full CRUD, SLA tiers, sentiment detection, auto-assign, status transitions |
| **bizclaw-accounting** | 4 | ✅ FULL | Double-entry ledger, VAT/PIT/CIT, Trial Balance, Cash Flow, payment records |
| **bizclaw-inventory** | 9 | ✅ FULL | Multi-warehouse, reserve/release/fulfill, low stock alerts, stock valuation |
| **bizclaw-pos** | 8 | ✅ FULL | Full CRUD products, sale checkout, void/refund, category search |
| **bizclaw-knowledge** | - | ✅ FULL | Knowledge Graph visualization API |

**Tổng tests: 32 tests pass** 🎉

---

## 🔧 FEATURES ĐÃ HOÀN THIỆN

### bizclaw-support (6 tests)
- ✅ Customer CRUD + segment auto-calculation
- ✅ Ticket CRUD với status transitions validation
- ✅ Auto-assign theo keywords (10 Vietnamese keywords)
- ✅ SLA tiers: VIP (1h), Express (4h), Normal (8h), Economy (24h)
- ✅ Sentiment detection tiếng Việt (14 positive, 12 negative, 10 urgent words)
- ✅ SLA violation tracking
- ✅ Escalation rules với assign_to

### bizclaw-accounting (4 tests)
- ✅ Double-entry bookkeeping (11 accounts theo QĐ 48)
- ✅ VAT 10% inclusive/exclusive
- ✅ PIT progressive brackets (5-30%)
- ✅ CIT 20% corporate
- ✅ Trial Balance report
- ✅ Cash Flow calculation
- ✅ Payment received/paid records
- ✅ VAT report generation

### bizclaw-inventory (9 tests)
- ✅ Multi-warehouse management
- ✅ Stock receive/sell/adjust
- ✅ Reserve/Release/Fulfill reserved stock
- ✅ Low stock alerts (OutOfStock, LowStock)
- ✅ Reorder point tracking
- ✅ Stock valuation (quantity × cost)
- ✅ Warehouse CRUD

### bizclaw-pos (8 tests)
- ✅ Product CRUD (add/get/update/delete/activate/deactivate)
- ✅ Category-based product filtering
- ✅ Barcode lookup
- ✅ Sale workflow (create → add items → checkout)
- ✅ Void/Refund operations
- ✅ Daily summary
- ✅ Sales by date query
- ✅ Product search

### bizclaw-payment (3 tests + 1 doc)
- ✅ VietQR generation với CRC16 checksum
- ✅ VietQR payload parsing
- ✅ VND formatting với thousand separators (1.000.000 VND)
- ✅ 24 supported banks
- ✅ VietQR payment URL generation

---

## 🚧 Modules Đang phát triển

| Module | Status | Notes |
|--------|--------|-------|
| **bizclaw-hands** | In Progress | Autonomous automation agent |
| **bizclaw-cortex** | Partial | Agent orchestration |
| **bizclaw-skills** | Partial | MAMA skill marketplace |

---

## 📋 Stub Modules (Chưa implement)

| Module | Priority | Description |
|--------|----------|-------------|
| bizclaw-crm | Medium | Customer relationship management |
| bizclaw-marketing | Low | Campaign management |
| bizclaw-analytics | Medium | Business intelligence |

---

## 🎯 CORE BUSINESS FLOWS

### 1. Order to Cash (OTC)
```
POS Sale → Payment (VietQR/MoMo) → Inventory deduct → Accounting entry → Receipt
```

### 2. Procure to Pay (PTP)
```
Purchase Order → Receive → Payment → Inventory add → Accounting entry
```

### 3. Customer Support
```
Ticket (Zalo/Facebook) → Sentiment detection → Auto-assign → SLA tracking → Resolution
```

---

## 📁 PROJECT STRUCTURE

```
bizclaw/
├── Cargo.toml (workspace root)
├── Memory.md (this file)
├── crates/
│   ├── bizclaw-core/           # Core agent infrastructure
│   ├── bizclaw-payment/        # VietQR, MoMo, ZaloPay, format_vnd
│   ├── bizclaw-support/        # Ticket system, SLA, sentiment (6 tests)
│   ├── bizclaw-accounting/      # Bookkeeping, Trial Balance (4 tests)
│   ├── bizclaw-inventory/       # Stock management, warehouses (9 tests)
│   ├── bizclaw-pos/             # Point of Sale, receipts (8 tests)
│   ├── bizclaw-knowledge/       # Knowledge Graph
│   ├── bizclaw-skills/         # MAMA skill registry
│   ├── bizclaw-platform/       # Platform integration
│   ├── bizclaw-hands/          # Autonomous automation
│   └── ...
└── Memory.md
```

---

## 🔄 CONVENTIONS

### Code Style
- Variables/Functions: English
- Business Logic Comments: Vietnamese
- Error Messages: Tiếng Việt
- Test Names: Vietnamese

### Dependencies Available
- `uuid`, `chrono`, `serde`, `anyhow`, `thiserror`
- `sha2` (CRC16 calculations)
- `tokio` (async runtime)
- `reqwest` (HTTP client)

---

## 📊 METRICS

- **Total Crates:** 55+
- **Production Ready:** 6 modules
- **Test Coverage:** 32 tests pass
- **Languages:** Rust, TypeScript (platform), Python (sandbox)

---

## 🚀 NEXT STEPS

### High Priority
1. [ ] bizclaw-hands - Autonomous automation
2. [ ] bizclaw-cortex - Agent orchestration  
3. [ ] bizclaw-skills - MAMA skill marketplace

### Medium Priority  
1. [ ] Integration tests giữa các modules
2. [ ] bizclaw-crm module
3. [ ] bizclaw-analytics module

### Low Priority
1. [ ] bizclaw-marketing
2. [ ] Mobile app integration

---

*End of Memory.md*
