# 🦞 BizClaw

**Nền Tảng AI Cho Doanh Nghiệp Một Người (OPC) Việt Nam** — Retail Edition

*Một người, một đội quân AI, vận hành toàn bộ cửa hàng Retail.*

<p align="center">
  <a href="https://github.com/nguyenduchoai/bizclaw/actions"><img src="https://img.shields.io/github/actions/workflow/status/nguyenduchoai/bizclaw/ci?style=flat-square" alt="Build"></a>
  <a href="https://github.com/nguyenduchoai/bizclaw/releases"><img src="https://img.shields.io/github/v/release/nguyenduchoai/bizclaw?style=flat-square" alt="Version"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square" alt="License"></a>
</p>

---

## 🎯 Retail OPC Platform - 100% Ready

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    BIZCLAW RETAIL OPC PLATFORM                             │
├─────────────────┬─────────────────┬─────────────────┬─────────────────────┤
│   POINT OF SALE │    PAYMENTS      │   INVENTORY     │   CUSTOMERS         │
├─────────────────┼─────────────────┼─────────────────┼─────────────────────┤
│ POS Agent       │ VietQR Agent    │ Stock Manager  │ CRM Agent          │
│ Receipt Gen     │ MoMo/ZaloPay    │ Alerts        │ Support Ticket    │
│ Barcode Scan    │ Banking API     │ Warehouse     │ Customer Segment  │
├─────────────────┴─────────────────┴─────────────────┴─────────────────────┤
│                         ACCOUNTING & COMPLIANCE                            │
├─────────────────┬─────────────────┬─────────────────┬─────────────────────┤
│   ACCOUNTING     │    INVOICING    │   BANKING       │   REPORTING       │
├─────────────────┼─────────────────┼─────────────────┼─────────────────────┤
│ Ledger          │ VAT Invoice     │ Bank Transfer  │ Sales Report    │
│ Tax Calculator   │ E-Invoice API  │ Balance Check │ Daily Summary    │
│ Profit/Loss     │ VietQR Payment │ Batch Pay    │ Customer Analytics│
└─────────────────┴─────────────────┴─────────────────┴─────────────────────┘
```

### ✅ 100% OPC Modules Complete

| Module | Agent/Crate | Status |
|--------|-------------|--------|
| **Point of Sale** | bizclaw-pos | ✅ |
| **Payments** | bizclaw-payment | ✅ |
| **Inventory** | bizclaw-inventory | ✅ |
| **Customer Support** | bizclaw-support | ✅ |
| **Accounting** | bizclaw-accounting | ✅ |
| **E-Invoice** | bizclaw-einvoice | ✅ |
| **Banking** | bizclaw-banking | ✅ |
| **Proposals** | bizclaw-proposal | ✅ |
| **Marketing** | bizclaw-outreach | ✅ |
| **Analytics** | bizclaw-analytics | ✅ |

---

## ✨ Tính Năng Hoàn Chỉnh

### 🏪 Retail Operations
- **POS Agent** - Bán hàng tại quầy, quét mã vạch
- **Inventory** - Theo dõi tồn kho, cảnh báo hết hàng
- **Customer** - CRM, phân loại khách hàng VIP/Regular

### 💳 Payments & Banking
- **VietQR** - Thanh toán QR code tức thì
- **MoMo/ZaloPay** - Ví điện tử
- **Banking API** - Chuyển khoản tự động
- **E-Invoice** - Hóa đơn điện tử VNPT/Viettel/MISA

### 📊 Accounting & Compliance
- **Bookkeeping** - Sổ sách kế toán kép
- **VAT Reports** - Báo cáo thuế hàng quý
- **Financial Statements** - Bảng cân đối, P&L tự động

### 📈 Marketing & Sales
- **Outreach** - Zalo, Email marketing tự động
- **Analytics** - Dashboard KPIs, báo cáo doanh thu
- **Proposals** - Báo giá, hợp đồng

---

## 💰 Tiết Kiệm Chi Phí

| Trước BizClaw | Sau BizClaw | Tiết Kiệm |
|---------------|------------|-----------|
| Thu ngân: 8M | POS Agent | -100% |
| Kế toán: 5M | Accounting Agent | -100% |
| Nhân viên kho: 6M | Inventory Agent | -100% |
| CSKH: 5M | Support Agent | -100% |
| Marketing: 7M | Outreach Agent | -80% |
| **Tổng: 31M/tháng** | **~2M/tháng** | **-94%** |

---

## 🚀 Quick Start

```bash
# Build
git clone https://github.com/nguyenduchoai/bizclaw.git
cd bizclaw
cargo build --release

# Chạy POS
./target/release/bizclaw run --agent pos

# API Gateway
./target/release/bizclaw serve --port 3000
```

### Docker

```bash
docker run -d --name bizclaw-retail \
  -p 3000:3000 -p 8080:8080 \
  -v bizclaw-data:/data \
  nguyenduchoai/bizclaw:latest
```

---

## 📁 59+ Rust Crates

```
bizclaw/
├── Retail Suite
│   ├── bizclaw-pos/              # Point of Sale
│   ├── bizclaw-payment/          # VietQR, MoMo, ZaloPay
│   ├── bizclaw-inventory/        # Stock management
│   ├── bizclaw-support/         # Customer tickets
│   ├── bizclaw-accounting/      # Bookkeeping
│   ├── bizclaw-proposal/        # Quotes
│   ├── bizclaw-einvoice/        # VN e-invoice
│   └── bizclaw-banking/         # Bank transfers
├── Marketing Suite
│   ├── bizclaw-outreach/        # Zalo, Email
│   ├── bizclaw-analytics/       # Dashboard
│   └── bizclaw-channels/       # Multi-channel
├── AI Suite
│   ├── bizclaw-brain/          # Local AI (llama.cpp)
│   ├── bizclaw-agent/          # Multi-agent
│   └── bizclaw-memory/         # Vector storage
└── desktop/                    # Tauri app
```

---

## � Configuration

```toml
# config.toml
[retail]
store_name = "Cửa Hàng Mẫu"
tax_code = "0123456789"
address = "123 Đường ABC, Quận 1, TP.HCM"

[banking]
vietinbank_account = "1234567890"
vietqr_enabled = true

[invoice]
provider = "vnpt"
username = "your_username"
password = "your_password"

[inventory]
low_stock_alert = 10
auto_restock = true
```

---

## 📚 Documentation

| Tài Liệu | Mô Tả |
|----------|--------|
| [OPC Platform Plan](docs/BIZCLAW_OPC_PLATFORM_VIETNAM.md) | Business plan |
| [Demo Case Study](docs/DEMO_OPC_CASE_STUDY.md) | MINA's Boutique |
| [Competitive Analysis](docs/COMPETITIVE_ANALYSIS_BIZCLAW_VS_AGENCY_AGENTS.md) | vs competitors |

---

## 🛣️ Roadmap

- [x] **Q1 2025**: Retail OPC Suite ✅
- [ ] **Q2 2025**: Desktop App + Mobile POS
- [ ] **Q3 2025**: E-commerce Integration
- [ ] **Q4 2025**: Enterprise Features

---

## 📄 License

MIT License

---

<p align="center">
  <strong>Made with ❤️ for Vietnamese Retailers</strong>
</p>
