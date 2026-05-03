#!/bin/bash
# Auto-reset Demo Tenant every 4 hours

DB_PATH="/root/.bizclaw/platform.db"
LOG_FILE="/var/log/demo-reset.log"

echo "[$(date)] Starting demo tenant reset..." >> $LOG_FILE

# Check if demo tenant exists
DEMO_EXISTS=$(sqlite3 "$DB_PATH" "SELECT COUNT(*) FROM tenants WHERE id='demo-001';")

if [ "$DEMO_EXISTS" = "1" ]; then
    # Reset demo tenant data
    sqlite3 "$DB_PATH" "UPDATE tenants SET expires_at = datetime('now', '+4 hours') WHERE id='demo-001';"
    sqlite3 "$DB_PATH" "DELETE FROM conversations WHERE tenant_id='demo-001';" 2>/dev/null || true
    sqlite3 "$DB_PATH" "DELETE FROM messages WHERE tenant_id='demo-001';" 2>/dev/null || true
    sqlite3 "$DB_PATH" "UPDATE tenants SET status='running', updated_at=datetime('now') WHERE id='demo-001';"
    
    echo "[$(date)] Demo tenant reset completed. Next reset in 4 hours." >> $LOG_FILE
else
    # Recreate demo tenant if deleted
    sqlite3 "$DB_PATH" "INSERT INTO tenants (id, name, slug, status, plan, expires_at) VALUES ('demo-001', 'BizClaw Demo', 'demo', 'running', 'professional', datetime('now', '+4 hours'));"
    sqlite3 "$DB_PATH" "INSERT INTO users (id, email, password_hash, role, tenant_id, status) VALUES ('demo-user-001', 'demo@bizclaw.vn', '\$2b\$12\$LQv3c1yqBWVJcKD0EKjH0OKFQQz4xGzv7GJJ1V3xKZQp0pQ0KV3Cy', 'admin', 'demo-001', 'active');"
    echo "[$(date)] Demo tenant recreated." >> $LOG_FILE
fi
