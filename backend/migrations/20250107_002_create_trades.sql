-- Create trades table
CREATE TABLE IF NOT EXISTS trades (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Trade Details
    symbol VARCHAR(20) NOT NULL,
    direction VARCHAR(10) NOT NULL CHECK (direction IN ('long', 'short')),
    entry_price DECIMAL(20, 8) NOT NULL,
    exit_price DECIMAL(20, 8),
    quantity DECIMAL(20, 8) NOT NULL,
    
    -- Timestamps
    entry_time TIMESTAMPTZ NOT NULL,
    exit_time TIMESTAMPTZ,
    
    -- P&L
    pnl DECIMAL(20, 8),
    pnl_percentage DECIMAL(10, 4),
    fees DECIMAL(20, 8) DEFAULT 0,
    
    -- Metadata
    notes TEXT,
    tags TEXT[] DEFAULT '{}',
    setup_type VARCHAR(100),
    mistakes TEXT[] DEFAULT '{}',
    emotions TEXT[] DEFAULT '{}',
    screenshots TEXT[] DEFAULT '{}',
    
    -- Additional Fields
    broker VARCHAR(100),
    account_id VARCHAR(100),
    status VARCHAR(20) DEFAULT 'open' CHECK (status IN ('open', 'closed', 'pending')),
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_trades_user_id ON trades(user_id);
CREATE INDEX IF NOT EXISTS idx_trades_entry_time ON trades(entry_time DESC);
CREATE INDEX IF NOT EXISTS idx_trades_symbol ON trades(symbol);
CREATE INDEX IF NOT EXISTS idx_trades_status ON trades(status);

-- Add comments
COMMENT ON TABLE trades IS 'Trading journal entries';
COMMENT ON COLUMN trades.direction IS 'long or short';
COMMENT ON COLUMN trades.status IS 'open, closed, pending';

