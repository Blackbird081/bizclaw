# 🦞 BizClaw

**Nền tảng AI ERP cho Doanh nghiệp một người (OPC) Việt Nam**

*Một người, một đội quân AI, vận hành toàn bộ cửa hàng.*

---

## 🚀 Triển khai nhanh

### Docker (Khuyến nghị)

```bash
# Pull và chạy
docker run -d \
  --name bizclaw \
  -p 3000:3000 \
  -e DATABASE_URL=postgres://user:pass@host:5432/bizclaw \
  -e DOMAIN=https://shop-cua-ban.com \
  nguyenduchoai/bizclaw:latest

# Hoặc dùng docker-compose
curl -O https://raw.githubusercontent.com/nguyenduchoai/bizclaw/main/docker-compose.yml
docker-compose up -d
```

### Railway (Cloud tự động)

[![Deploy on Railway](https://railway.app/button.svg)](https://railway.app/new/template?template=https://github.com/nguyenduchoai/bizclaw)

### Render

[![Deploy to Render](https://render.com/badge.svg)](https://render.com/deploy?repo=https://github.com/nguyenduchoai/bizclaw)

---

## 📦 Modules sẵn sàng

| Module | Mô tả | Trạng thái |
|--------|--------|-------------|
| **POS** | Bán hàng, in hóa đơn | ✅ Production |
| **Inventory** | Quản lý kho, cảnh báo tồn kho | ✅ Production |
| **Support** | Ticket khách hàng, SLA | ✅ Production |
| **Accounting** | Sổ sách, VAT, Báo cáo | ✅ Production |
| **Payment** | VietQR, MoMo, ZaloPay | ✅ Production |

---

## 🌐 Cấu hình Domain

### Single Tenant (1 cửa hàng)

```env
# .env
DOMAIN=https://shop-cua-ban.com
TENANT_MODE=single
DATABASE_URL=postgres://...
```

### Custom Domain (Cloudflare/Railway)

1. Thêm CNAME record trỏ đến deployment URL
2. Enable SSL tự động
3. Cấu hình in `docker-compose.yml`:

```yaml
services:
  bizclaw:
    environment:
      - DOMAIN=${DOMAIN}
      - TENANT_MODE=single
    labels:
      - traefik.http.routers.bizclaw.rule=Host(`shop-cua-ban.com`)
```

---

## 📊 Workflow vận hành

```
Khách hàng order
    ↓
POS tạo đơn → Trừ kho
    ↓
Thanh toán VietQR/MoMo
    ↓
Accounting ghi nhận doanh thu
    ↓
Support ticket nếu có khiếu nại
```

---

## 🔧 Cấu hình môi trường

| Biến | Mô tả | Ví dụ |
|-------|--------|--------|
| `DOMAIN` | Domain cửa hàng | `https://shop-cua-ban.com` |
| `DATABASE_URL` | PostgreSQL connection | `postgres://...` |
| `VIETQR_BANK_ID` | Mã ngân hàng | `970436` |
| `VIETQR_MERCHANT_ID` | Merchant ID | `12345678901` |
| `ZALO_APP_ID` | Zalo App ID | `123456789` |
| `ZALO_APP_SECRET` | Zalo Secret | `xxx` |

---

## 📈 Dashboard KPIs

- **Doanh thu hôm nay** - Tổng sales
- **Đơn hàng** - Số lượng transactions
- **Tồn kho thấp** - Cảnh báo reorder
- **Ticket chưa giải quyết** - SLA breaches

---

## 🔒 Bảo mật

- SSL/TLS auto (Let's Encrypt)
- Environment variables cho secrets
- PostgreSQL với connection pooling
- Rate limiting trên API endpoints

---

## 📞 Hỗ trợ

- **Document**: [docs/](docs/)
- **Issues**: [GitHub Issues](https://github.com/nguyenduchoai/bizclaw/issues)
- **Email**: support@bizclaw.io

---

## 📄 License

MIT License - Free cho sử dụng thương mại
