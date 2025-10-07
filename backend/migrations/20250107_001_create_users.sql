-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    email_verified BOOLEAN DEFAULT FALSE,
    password_hash VARCHAR(255) NOT NULL,
    
    -- Stripe Integration
    stripe_customer_id VARCHAR(255) UNIQUE,
    subscription_status VARCHAR(50) DEFAULT 'none',
    subscription_tier VARCHAR(50) DEFAULT 'none',
    subscription_interval VARCHAR(20),
    
    permissions TEXT[] DEFAULT '{}',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    last_login TIMESTAMP
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_stripe_customer ON users(stripe_customer_id);

-- Add comments
COMMENT ON TABLE users IS 'User accounts with Stripe subscription integration';
COMMENT ON COLUMN users.subscription_status IS 'active, canceled, past_due, trialing, none';
COMMENT ON COLUMN users.subscription_tier IS 'none, paid';
COMMENT ON COLUMN users.subscription_interval IS 'month, month_6, year';

