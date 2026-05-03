# 🦞 BizClaw Cloud

**Nền tảng AI vận hành cho OPC Việt Nam - Cloud Only**

*Đăng ký, dùng ngay. Không cần cài đặt.*

---

## 🎯 BizClaw Cloud giúp gì?

### 📱 Retail (Bán lẻ)

Bạn bán hàng trên TikTok Shop, Shopee, Zalo... và muốn quản lý tất cả ở một nơi.

```
Khách đặt trên TikTok Shop / Shopee / Zalo
        ↓
MAMA nhận đơn → Tạo ticket
        ↓
Bạn duyệt đơn → Cập nhật kho
        ↓
Thanh toán VietQR / MoMo
        ↓
Accounting tự động ghi nhận doanh thu
```

### 🏨 Tourist (Du lịch)

Bạn nhận booking từ Traveloka, Agoda, Direct... và muốn quản lý khách.

```
Khách đặt phòng / tour
        ↓
MAMA tạo ticket → Gửi xác nhận
        ↓
Bạn phân công nhân sự
        ↓
Check-in / Check-out
        ↓
Accounting tự động
```

---

## 🤖 MAMA AI

MAMA là AI agent tổng quản:
- **Tiếp nhận** ticket từ khách
- **Tự động** phân loại theo nội dung
- **Cảnh báo** khi có vấn đề
- **Gợi ý** giải pháp

---

## � Triển khai

### Đăng ký ngay

1. Truy cập **bizclaw.io**
2. Tạo tài khoản
3. Chọn gói phù hợp
4. Bắt đầu sử dụng

### Tự deploy (Advanced)

```bash
# Deploy lên Railway/Render
git clone https://github.com/nguyenduchoai/bizclaw-cloud
cd bizclaw-cloud
railway up
```

---

## �📦 Modules

| Module | Mô tả |
|--------|--------|
| **Support** | Ticket, tin nhắn, SLA |
| **Inventory** | Quản lý kho |
| **Accounting** | Sổ sách, VAT |
| **Payment** | VietQR, MoMo, ZaloPay |

---

## � Bảng giá

| Gói | Giá | Phù hợp |
|------|------|----------|
| **Starter** | Miễn phí | 1 người |
| **Pro** | 299K/tháng | 1-3 người |
| **Business** | 799K/tháng | 3-10 người |

---

## 🌐 API Endpoints

Truy cập `https://api.bizclaw.io/v1/`

```bash
# Support
GET  /v1/support/tickets
POST /v1/support/tickets

# Inventory
GET  /v1/inventory/items
POST /v1/inventory/items

# Accounting
GET  /v1/accounting/reports/balance-sheet
GET  /v1/accounting/reports/income-statement

# Payment
POST /v1/payments/vietqr/generate
GET  /v1/payments/vietqr/banks
```

---

## 📄 License

MIT License
