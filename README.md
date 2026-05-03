# 🦞 BizClaw

**Nền tảng AI vận hành cho OPC Việt Nam**

*MAMA AI giúp bạn quản lý khách hàng, đơn hàng, kho hàng... không cần thuê nhân sự.*

---

## 🎯 BizClaw giúp gì?

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

## 📦 Modules

| Module | Mô tả |
|--------|--------|
| **Support** | Ticket, tin nhắn, SLA |
| **Inventory** | Quản lý kho |
| **Accounting** | Sổ sách, VAT |
| **Payment** | VietQR, MoMo, ZaloPay |

---

## 🚀 Triển khai

### Docker

```bash
docker run -d \
  --name bizclaw \
  -p 3000:3000 \
  -e DATABASE_URL=postgres://user:pass@host:5432/bizclaw \
  nguyenduchoai/bizclaw:latest
```

### Railway

[![Deploy on Railway](https://railway.app/button.svg)](https://railway.app/new/template?template=https://github.com/nguyenduchoai/bizclaw)

---

## 🌐 Domain

```env
DOMAIN=https://shop-cua-ban.com
TENANT_MODE=single
DATABASE_URL=postgres://...
```

---

## 📈 Dashboard KPIs

- **Đơn hàng hôm nay**
- **Ticket chưa xử lý**
- **Tồn kho thấp**
- **Doanh thu**

---

## 📄 License

MIT License
