# 🦞 BizClaw

**AI Agent Platform cho Doanh Nghiệp Một Người (OPC) Việt Nam**

*Một người, một đội quân AI, vận hành mọi nghiệp vụ doanh nghiệp.*

<p align="center">
  <a href="https://github.com/nguyenduchoai/bizclaw/actions"><img src="https://img.shields.io/github/actions/workflow/status/nguyenduchoai/bizclaw/ci?style=flat-square" alt="Build"></a>
  <a href="https://github.com/nguyenduchoai/bizclaw/releases"><img src="https://img.shields.io/github/v/release/nguyenduchoai/bizclaw?style=flat-square" alt="Version"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square" alt="License"></a>
  <a href="https://crates.io/crates/bizclaw-core"><img src="https://img.shields.io/crates/v/bizclaw-core?style=flat-square" alt="Crate"></a>
</p>

---

## 🎯 Tại Sao Chọn BizClaw?

| So Sánh | BizClaw | agency-agents | Zapier | Make.com |
|----------|---------|---------------|--------|-----------|
| **Giá** | Freemium | $49/mo | $49/mo | $59/mo |
| **Tiếng Việt** | ✅ | ❌ | ❌ | ❌ |
| **Hóa đơn VAT** | ✅ | ❌ | ❌ | ❌ |
| **Hợp đồng VN** | ✅ | ❌ | ❌ | ❌ |
| **Self-hosted** | ✅ | ❌ | ❌ | ❌ |
| **Chạy Offline** | ✅ | ❌ | ❌ | ❌ |

---

## ✨ Tính Năng Nổi Bật

### 🇻🇳 Vietnamese-First
- **Zalo Integration** - Gửi tin nhắn, quản lý OA
- **VAT Invoice** - Hóa đơn điện tử theo quy định VN
- **VietQR Payment** - Thanh toán tự động
- **Vietnamese Contracts** - Mẫu hợp đồng theo Luật VN
- **Legal Compliance** - Kiểm tra compliance tự động

### 🤖 AI Agents
- **18+ AI Providers** - OpenAI, Anthropic, Gemini, DeepSeek, Groq, Ollama, MiniMax
- **Local AI** - Chạy llama.cpp inference offline
- **Multi-agent** - 51+ agents chuyên biệt
- **Skills System** - Khả năng mở rộng không giới hạn

### � Integrations
- **Channels** - Zalo, Telegram, Discord, Slack, WhatsApp, Email, Webhook
- **Social** - TikTok, Facebook, Instagram
- **E-commerce** - Shopee, TikTok Shop
- **CRM** - Quản lý khách hàng toàn diện

### 🛡️ Enterprise-Ready
- **Security** - AES-256, Prompt injection detection
- **Self-hosted** - 100% private, không phụ thuộc cloud
- **Audit Logs** - Theo dõi mọi hoạt động
- **API Key Vault** - Quản lý secrets an toàn

---

## 🏢 OPC Suite - Giải Pháp Cho Doanh Nghiệp 1 Người

```
┌─────────────────────────────────────────────────────────────────┐
│                    BIZCLAW OPC PLATFORM                          │
├───────────────┬───────────────┬───────────────┬────────────────────┤
│   MARKETING   │   SALES      │   FINANCE    │   OPERATIONS       │
├───────────────┼───────────────┼───────────────┼────────────────────┤
│ Outreach Agent│ CRM Agent    │ Invoice Agent│ Scheduler Agent   │
│ Social Agent │ Ecommerce    │ Legal Agent │ Browser Agent    │
│ Analytics    │ Channels     │ Analytics   │ Memory Agent     │
│ Media Gen    │ Payment      │ VietQR      │ Skills Runner    │
└───────────────┴───────────────┴───────────────┴────────────────────┘
```

### 💰 Tiết Kiệm Chi Phí

| Trước BizClaw | Sau BizClaw | Tiết Kiệm |
|---------------|------------|-----------|
| Nhân viên marketing: 7M | BizClaw Pro: 0.5M | -93% |
| Kế toán: 3M | Invoice Agent | -100% |
| Sales: 8M | CRM + Channels | -100% |
| **Tổng: 18M/tháng** | **~1M/tháng** | **-94%** |

---

## 🚀 Quick Start

### Cài Đặt Nhanh

```bash
# Clone & build
git clone https://github.com/nguyenduchoai/bizclaw.git
cd bizclaw
cargo build --release

# Chạy desktop app
./target/release/bizclaw-desktop

# Hoặc chạy CLI
./target/release/bizclaw run --agent outreach
```

### Docker

```bash
docker run -d --name bizclaw \
  -p 3000:3000 -p 8080:8080 \
  -v bizclaw-data:/data \
  nguyenduchoai/bizclaw:latest
```

### Configuration

```toml
# config.toml
[opc]
name = "My Business"
type = "ecommerce"

[outreach]
zalo_enabled = true
zalo_app_id = "YOUR_ZALO_APP_ID"

[invoice]
vat_enabled = true
tax_code = "YOUR_TAX_CODE"
vietqr_bank = "Vietcombank"
vietqr_account = "1234567890"

[analytics]
kpis = ["revenue", "orders", "customers"]
```

---

## 📦 51+ Crates (AI Agents)

| Category | Crates | Mô Tả |
|----------|--------|--------|
| **AI & Brain** | brain, agent, providers, agi, hermes | Neural network, local inference |
| **Business Suite** | outreach, invoice, legal, analytics | OPC core agents |
| **Channels** | channels, social, media, email | Multi-channel comms |
| **Operations** | crm, ecommerce, scheduler, office | Business operations |
| **Infrastructure** | memory, security, gateway, mcp | Platform foundation |

---

## 📁 Project Structure

```
bizclaw/
├── crates/                    # 51 Rust crates
│   ├── bizclaw-brain/        # Neural network engine (llama.cpp)
│   ├── bizclaw-agent/        # Agent orchestration (A2A protocol)
│   ├── bizclaw-outreach/     # Outreach automation
│   ├── bizclaw-invoice/      # Invoice & VAT (Vietnam)
│   ├── bizclaw-legal/       # Contracts & compliance
│   ├── bizclaw-analytics/    # KPIs & dashboards
│   ├── bizclaw-channels/    # Zalo, Email, Slack...
│   ├── bizclaw-memory/       # Vector + SQLite storage
│   ├── bizclaw-mcp/         # Model Context Protocol
│   └── ... (45+ more crates)
├── desktop/                  # Tauri desktop app
│   └── src-ui/              # React + TypeScript frontend
├── docs/                    # Documentation
│   ├── BIZCLAW_OPC_PLATFORM_VIETNAM.md    # Business plan
│   ├── DEMO_OPC_CASE_STUDY.md             # Use case
│   └── COMPETITIVE_ANALYSIS.md           # vs competitors
└── Dockerfile
```

---

## 🔬 Demo: MINA's Boutique

Xem [Demo OPC Case Study](docs/DEMO_OPC_CASE_STUDY.md) để biết cách một cửa hàng thời trang có thể:

- ✅ Gửi 500 tin Zalo trong 5 phút
- ✅ Xuất hóa đơn VAT tự động
- ✅ Tạo hợp đồng thuê mặt bằng
- ✅ Theo dõi KPIs real-time

**Kết quả:** Tiết kiệm 17M/tháng, tăng 81% doanh thu

---

## 🤝 Contributing

```bash
# Fork và clone
git clone https://github.com/YOUR_USER/bizclaw.git
cd bizclaw

# Tạo feature branch
git checkout -b feature/my-agent

# Build và test
cargo build --release
cargo test -p bizclaw-outreach

# Commit và push
git commit -m "feat: add new OPC agent"
git push origin feature/my-agent
```

---

## 📚 Documentation

| Tài Liệu | Mô Tả |
|----------|--------|
| [OPC Platform Plan](docs/BIZCLAW_OPC_PLATFORM_VIETNAM.md) | Business plan chi tiết |
| [Demo Case Study](docs/DEMO_OPC_CASE_STUDY.md) | Use case mẫu |
| [Competitive Analysis](docs/COMPETITIVE_ANALYSIS_BIZCLAW_VS_AGENCY_AGENTS.md) | So sánh với competitors |
| [ROADMAP](docs/ROADMAP.md) | Lộ trình phát triển |

---

## 🛣️ Roadmap

- [x] **Q1 2025**: OPC Suite Launch ✅
  - Outreach Agent
  - Invoice Agent  
  - Legal Agent
  - Analytics Agent

- [ ] **Q2 2025**: Desktop App
  - System tray
  - Push notifications
  - Voice I/O

- [ ] **Q3 2025**: Skills Marketplace
  - Web UI
  - Agent templates
  - Community features

- [ ] **Q4 2025**: Mobile App
  - React Native
  - Offline support
  - Widgets

---

## 📄 License

MIT License - Xem [LICENSE](LICENSE)

---

## 🙏 Credits

Xem [CREDITS.md](CREDITS.md) để biết danh sách contributors và dependencies.

---

<p align="center">
  <strong>Made with ❤️ in Vietnam</strong>
  <br>
  <a href="https://github.com/nguyenduchoai/bizclaw">GitHub</a> •
  <a href="https://crates.io/crates/bizclaw-core">Crates.io</a> •
  <a href="https://discord.gg/bizclaw">Discord</a>
</p>
