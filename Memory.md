# BizClaw - Persistent Memory

> **Last Updated:** 2025-01-17  
> **Version:** 1.1.7  
> **Language:** Vietnamese OPC (One Person Company) ERP

---

## 🏢 TỔNG QUAN HỆ THỐNG

### BizClaw (Single-tenant / Local)
- **Repo:** https://github.com/nguyenduchoai/bizclaw
- **Mục đích:** Library/SDK cho developers muốn self-host
- **Tech:** Rust, Axum, SQLx, PostgreSQL

### BizClaw Cloud (SaaS)
- **Repo:** https://github.com/nguyenduchoai/bizclaw-cloud  
- **Mục đích:** SaaS cho OPC Việt Nam
- **Tech:** HTML/CSS/JS + Rust backend

---

## 📁 PROJECT STRUCTURE

### bizclaw (Library/SDK)
```
bizclaw/
├── Cargo.toml (workspace)
├── crates/
│   ├── bizclaw-gateway/    # HTTP/WebSocket API
│   ├── bizclaw-agent/      # AI Agent engine
│   ├── bizclaw-channels/   # Zalo, Telegram, Email
│   ├── bizclaw-support/    # Support modules
│   ├── bizclaw-accounting/  # Accounting modules
│   ├── bizclaw-inventory/ # Inventory modules
│   ├── bizclaw-payment/    # VietQR, MoMo, ZaloPay
│   └── ...
├── migrations/            # SQL schema
└── Memory.md
```

### bizclaw-cloud (SaaS)
```
bizclaw-cloud/
├── index.html       # Landing page
├── login.html      # Auth
├── register.html   # Registration
├── dashboard.html  # Main dashboard
└── README.md
```

---

## 🚀 DEPLOYMENT

### Local (bizclaw)
```bash
cargo build --release
./target/release/bizclaw serve
```

### Cloud (bizclaw-cloud)
```bash
# Static files deploy lên Vercel/Netlify
# Backend deploy lên Railway/Render
```

---

## 📝 NOTES

### Cần làm thêm cho Cloud:
1. Backend API server (Rust/Axum)
2. PostgreSQL database
3. Auth JWT implementation
4. Billing integration (VNPay/MoMo)
5. Email verification
6. Password reset

### Pricing:
- Starter: Miễn phí
- Pro: 299K/tháng
- Business: 799K/tháng

---

*End of Memory.md*
