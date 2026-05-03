//! Database migrations for BizClaw modules
//! 
//! This module provides SQL migrations for the core business modules.

use sqlx::PgPool;

pub async fn run_migrations(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    // Tenants
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS tenants (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(255) NOT NULL,
            slug VARCHAR(100) UNIQUE NOT NULL,
            domain VARCHAR(255),
            plan VARCHAR(50) DEFAULT 'free',
            settings JSONB DEFAULT '{}',
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )
    "#).execute(pool).await?;

    // Users
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255),
            name VARCHAR(255),
            role VARCHAR(50) DEFAULT 'user',
            metadata JSONB DEFAULT '{}',
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )
    "#).execute(pool).await?;

    // Customers
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS customers (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255),
            phone VARCHAR(50),
            zalo_id VARCHAR(100),
            facebook_id VARCHAR(100),
            telegram_id VARCHAR(100),
            segment VARCHAR(50) DEFAULT 'new',
            lifetime_value BIGINT DEFAULT 0,
            total_orders INT DEFAULT 0,
            total_spent BIGINT DEFAULT 0,
            sentiment_score DECIMAL(3,2) DEFAULT 0,
            tags TEXT[] DEFAULT '{}',
            metadata JSONB DEFAULT '{}',
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )
    "#).execute(pool).await?;

    // Tickets
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS tickets (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
            customer_id UUID REFERENCES customers(id),
            channel VARCHAR(50) NOT NULL,
            subject VARCHAR(500) NOT NULL,
            status VARCHAR(50) DEFAULT 'open',
            priority VARCHAR(50) DEFAULT 'normal',
            sla_due TIMESTAMPTZ,
            assignees TEXT[] DEFAULT '{}',
            tags TEXT[] DEFAULT '{}',
            metadata JSONB DEFAULT '{}',
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )
    "#).execute(pool).await?;

    // Ticket Messages
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS ticket_messages (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            ticket_id UUID REFERENCES tickets(id) ON DELETE CASCADE,
            sender_id VARCHAR(255) NOT NULL,
            sender_name VARCHAR(255),
            content TEXT NOT NULL,
            attachments TEXT[] DEFAULT '{}',
            created_at TIMESTAMPTZ DEFAULT NOW()
        )
    "#).execute(pool).await?;

    // Warehouses
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS warehouses (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
            code VARCHAR(50) NOT NULL,
            name VARCHAR(255) NOT NULL,
            address TEXT,
            is_default BOOLEAN DEFAULT false,
            metadata JSONB DEFAULT '{}',
            created_at TIMESTAMPTZ DEFAULT NOW()
        )
    "#).execute(pool).await?;

    // Inventory Items
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS inventory_items (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
            warehouse_id UUID REFERENCES warehouses(id),
            sku VARCHAR(100) NOT NULL,
            name VARCHAR(255) NOT NULL,
            category VARCHAR(100),
            barcode VARCHAR(100),
            quantity INT DEFAULT 0,
            available INT DEFAULT 0,
            reserved INT DEFAULT 0,
            reorder_point INT DEFAULT 10,
            cost BIGINT DEFAULT 0,
            metadata JSONB DEFAULT '{}',
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW(),
            UNIQUE(tenant_id, sku)
        )
    "#).execute(pool).await?;

    // Stock Transactions
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS stock_transactions (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
            item_id UUID REFERENCES inventory_items(id),
            warehouse_id UUID REFERENCES warehouses(id),
            quantity_change INT NOT NULL,
            quantity_before INT NOT NULL,
            quantity_after INT NOT NULL,
            reason VARCHAR(100),
            reference_id VARCHAR(255),
            created_at TIMESTAMPTZ DEFAULT NOW()
        )
    "#).execute(pool).await?;

    // Accounts (Chart of Accounts)
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS accounts (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
            code VARCHAR(20) NOT NULL,
            name VARCHAR(255) NOT NULL,
            account_type VARCHAR(50) NOT NULL,
            parent_id UUID REFERENCES accounts(id),
            is_active BOOLEAN DEFAULT true,
            metadata JSONB DEFAULT '{}',
            created_at TIMESTAMPTZ DEFAULT NOW(),
            UNIQUE(tenant_id, code)
        )
    "#).execute(pool).await?;

    // Accounting Transactions
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS accounting_transactions (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
            transaction_date DATE NOT NULL,
            description VARCHAR(500),
            reference VARCHAR(255),
            metadata JSONB DEFAULT '{}',
            created_at TIMESTAMPTZ DEFAULT NOW()
        )
    "#).execute(pool).await?;

    // Transaction Entries
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS transaction_entries (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            transaction_id UUID REFERENCES accounting_transactions(id) ON DELETE CASCADE,
            account_id UUID REFERENCES accounts(id),
            debit BIGINT DEFAULT 0,
            credit BIGINT DEFAULT 0,
            memo VARCHAR(255)
        )
    "#).execute(pool).await?;

    // Payments
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS payments (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
            order_id VARCHAR(255),
            customer_id UUID REFERENCES customers(id),
            amount BIGINT NOT NULL,
            currency VARCHAR(10) DEFAULT 'VND',
            method VARCHAR(50) NOT NULL,
            status VARCHAR(50) DEFAULT 'pending',
            provider VARCHAR(50),
            provider_ref VARCHAR(255),
            metadata JSONB DEFAULT '{}',
            paid_at TIMESTAMPTZ,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )
    "#).execute(pool).await?;

    // Products
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS products (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
            sku VARCHAR(100) NOT NULL,
            name VARCHAR(255) NOT NULL,
            category VARCHAR(100),
            barcode VARCHAR(100),
            price BIGINT NOT NULL,
            cost BIGINT DEFAULT 0,
            stock_qty INT DEFAULT 0,
            is_active BOOLEAN DEFAULT true,
            metadata JSONB DEFAULT '{}',
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW(),
            UNIQUE(tenant_id, sku)
        )
    "#).execute(pool).await?;

    // Sales
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS sales (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
            sale_number VARCHAR(50) NOT NULL,
            customer_id UUID REFERENCES customers(id),
            cashier_id VARCHAR(255),
            subtotal BIGINT DEFAULT 0,
            discount BIGINT DEFAULT 0,
            vat_amount BIGINT DEFAULT 0,
            total BIGINT NOT NULL,
            payment_method VARCHAR(50),
            status VARCHAR(50) DEFAULT 'completed',
            metadata JSONB DEFAULT '{}',
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )
    "#).execute(pool).await?;

    // Sale Items
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS sale_items (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            sale_id UUID REFERENCES sales(id) ON DELETE CASCADE,
            product_id UUID REFERENCES products(id),
            product_name VARCHAR(255),
            quantity INT NOT NULL,
            unit_price BIGINT NOT NULL,
            discount BIGINT DEFAULT 0,
            total BIGINT NOT NULL
        )
    "#).execute(pool).await?;

    // Create indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_customers_tenant ON customers(tenant_id)").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tickets_tenant ON tickets(tenant_id)").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tickets_status ON tickets(status)").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_inventory_tenant ON inventory_items(tenant_id)").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_payments_tenant ON payments(tenant_id)").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_tenant ON products(tenant_id)").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_sales_tenant ON sales(tenant_id)").execute(pool).await?;

    tracing::info!("All BizClaw migrations completed");
    Ok(())
}
