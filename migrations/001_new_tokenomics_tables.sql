-- Migration: New Tokenomics Tables
-- Description: Create tables for the new ST/UT tokenomics model
-- Date: 2025-01-15

-- Users table for KYC and wallet management
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    phone_hash VARCHAR(64) UNIQUE NOT NULL,
    wallet_addr VARCHAR(42) UNIQUE,
    kyc_status VARCHAR(20) DEFAULT 'pending' CHECK (kyc_status IN ('not_required', 'pending', 'verified', 'rejected', 'expired')),
    full_name VARCHAR(255),
    email VARCHAR(255),
    t_shirt_size VARCHAR(10) CHECK (t_shirt_size IN ('XXS', 'XS', 'S', 'M', 'L', 'XL', 'XXL')),
    favorite_dish VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Sales table for ST token minting
CREATE TABLE IF NOT EXISTS sales (
    sale_id VARCHAR(255) PRIMARY KEY,
    node_id VARCHAR(255) NOT NULL,
    user_id INTEGER REFERENCES users(id),
    amount_gel DECIMAL(10,2) NOT NULL,
    st_units BIGINT NOT NULL,
    check_address VARCHAR(42) NOT NULL,
    activation_code_hash VARCHAR(64) NOT NULL,
    status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'processed', 'claimed', 'expired')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- ST mintings table for tracking token minting
CREATE TABLE IF NOT EXISTS st_mintings (
    mint_id VARCHAR(255) PRIMARY KEY,
    sale_id VARCHAR(255) REFERENCES sales(sale_id),
    units BIGINT NOT NULL,
    to_address VARCHAR(42) NOT NULL,
    transaction_hash VARCHAR(66) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- UT events table for tracking utility token awards
CREATE TABLE IF NOT EXISTS ut_events (
    event_id VARCHAR(255) PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    event_type VARCHAR(20) NOT NULL CHECK (event_type IN ('streaming', 'comment', 'share', 'like', 'view')),
    units BIGINT NOT NULL,
    reference VARCHAR(255) NOT NULL,
    platform VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- UT balances table (denormalized for fast reads)
CREATE TABLE IF NOT EXISTS ut_balances (
    user_id INTEGER PRIMARY KEY REFERENCES users(id),
    units BIGINT DEFAULT 0,
    voting_power BIGINT DEFAULT 0,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- ST balances table for security tokens
CREATE TABLE IF NOT EXISTS st_balances (
    user_id INTEGER PRIMARY KEY REFERENCES users(id),
    units BIGINT DEFAULT 0,
    kyc_status VARCHAR(20) DEFAULT 'pending' CHECK (kyc_status IN ('not_required', 'pending', 'verified', 'rejected', 'expired')),
    transfer_restricted BOOLEAN DEFAULT true,
    dividend_eligible BOOLEAN DEFAULT true,
    last_dividend_snapshot BIGINT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Conversion rounds table
CREATE TABLE IF NOT EXISTS conversion_rounds (
    round_id VARCHAR(255) PRIMARY KEY,
    total_pool BIGINT NOT NULL,
    total_ut_snapshot BIGINT NOT NULL,
    distributed BIGINT DEFAULT 0,
    status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'in_progress', 'completed', 'failed')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE
);

-- Conversion allocations table
CREATE TABLE IF NOT EXISTS conversion_allocations (
    id SERIAL PRIMARY KEY,
    round_id VARCHAR(255) REFERENCES conversion_rounds(round_id),
    user_id INTEGER REFERENCES users(id),
    allocated_units BIGINT NOT NULL,
    kyc_status VARCHAR(20) NOT NULL CHECK (kyc_status IN ('not_required', 'pending', 'verified', 'rejected', 'expired')),
    transaction_hash VARCHAR(66),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Governance proposals table
CREATE TABLE IF NOT EXISTS governance_proposals (
    proposal_id VARCHAR(255) PRIMARY KEY,
    proposer_id INTEGER REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    proposal_type VARCHAR(50) NOT NULL,
    status VARCHAR(20) DEFAULT 'active' CHECK (status IN ('active', 'passed', 'rejected', 'executed')),
    voting_start TIMESTAMP WITH TIME ZONE NOT NULL,
    voting_end TIMESTAMP WITH TIME ZONE NOT NULL,
    execution_delay_hours INTEGER DEFAULT 24,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Governance votes table
CREATE TABLE IF NOT EXISTS governance_votes (
    id SERIAL PRIMARY KEY,
    proposal_id VARCHAR(255) REFERENCES governance_proposals(proposal_id),
    voter_id INTEGER REFERENCES users(id),
    vote BOOLEAN NOT NULL,
    voting_power BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(proposal_id, voter_id)
);

-- Streaming sessions table for UT tracking
CREATE TABLE IF NOT EXISTS streaming_sessions (
    session_id VARCHAR(255) PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    stream_id VARCHAR(255) NOT NULL,
    platform VARCHAR(50) NOT NULL,
    start_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_time TIMESTAMP WITH TIME ZONE,
    duration_minutes INTEGER DEFAULT 0,
    ut_earned BIGINT DEFAULT 0,
    status VARCHAR(20) DEFAULT 'active' CHECK (status IN ('active', 'completed', 'abandoned')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Comments table for UT tracking
CREATE TABLE IF NOT EXISTS comments (
    comment_id VARCHAR(255) PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    stream_id VARCHAR(255) NOT NULL,
    platform VARCHAR(50) NOT NULL,
    content TEXT NOT NULL,
    ut_earned BIGINT DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Shares table for UT tracking
CREATE TABLE IF NOT EXISTS shares (
    share_id VARCHAR(255) PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    stream_id VARCHAR(255) NOT NULL,
    platform VARCHAR(50) NOT NULL,
    share_type VARCHAR(20) NOT NULL CHECK (share_type IN ('social', 'direct', 'embed')),
    ut_earned BIGINT DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Likes table for UT tracking
CREATE TABLE IF NOT EXISTS likes (
    like_id VARCHAR(255) PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    stream_id VARCHAR(255) NOT NULL,
    platform VARCHAR(50) NOT NULL,
    ut_earned BIGINT DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, stream_id, platform)
);

-- Reserved ST tracking table
CREATE TABLE IF NOT EXISTS reserved_st (
    id SERIAL PRIMARY KEY,
    total_reserved BIGINT DEFAULT 0,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Insert initial reserved ST record
INSERT INTO reserved_st (total_reserved) VALUES (0) ON CONFLICT DO NOTHING;

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_users_phone_hash ON users(phone_hash);
CREATE INDEX IF NOT EXISTS idx_users_wallet_addr ON users(wallet_addr);
CREATE INDEX IF NOT EXISTS idx_sales_node_id ON sales(node_id);
CREATE INDEX IF NOT EXISTS idx_sales_status ON sales(status);
CREATE INDEX IF NOT EXISTS idx_ut_events_user_id ON ut_events(user_id);
CREATE INDEX IF NOT EXISTS idx_ut_events_type ON ut_events(event_type);
CREATE INDEX IF NOT EXISTS idx_ut_events_created_at ON ut_events(created_at);
CREATE INDEX IF NOT EXISTS idx_conversion_allocations_round_id ON conversion_allocations(round_id);
CREATE INDEX IF NOT EXISTS idx_conversion_allocations_user_id ON conversion_allocations(user_id);
CREATE INDEX IF NOT EXISTS idx_governance_votes_proposal_id ON governance_votes(proposal_id);
CREATE INDEX IF NOT EXISTS idx_governance_votes_voter_id ON governance_votes(voter_id);
CREATE INDEX IF NOT EXISTS idx_streaming_sessions_user_id ON streaming_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_streaming_sessions_stream_id ON streaming_sessions(stream_id);
CREATE INDEX IF NOT EXISTS idx_comments_user_id ON comments(user_id);
CREATE INDEX IF NOT EXISTS idx_shares_user_id ON shares(user_id);
CREATE INDEX IF NOT EXISTS idx_likes_user_id ON likes(user_id);

-- Create triggers for updating timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_sales_updated_at BEFORE UPDATE ON sales
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_st_balances_updated_at BEFORE UPDATE ON st_balances
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Create function to update UT balances
CREATE OR REPLACE FUNCTION update_ut_balance()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO ut_balances (user_id, units, voting_power, last_updated)
    VALUES (NEW.user_id, NEW.units, NEW.units, NOW())
    ON CONFLICT (user_id) 
    DO UPDATE SET 
        units = ut_balances.units + NEW.units,
        voting_power = ut_balances.voting_power + NEW.units,
        last_updated = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_ut_balance_trigger AFTER INSERT ON ut_events
    FOR EACH ROW EXECUTE FUNCTION update_ut_balance();

-- Create function to update reserved ST
CREATE OR REPLACE FUNCTION update_reserved_st()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE reserved_st SET total_reserved = total_reserved + NEW.st_units, last_updated = NOW();
        RETURN NEW;
    ELSIF TG_OP = 'UPDATE' THEN
        IF OLD.status = 'pending' AND NEW.status = 'claimed' THEN
            UPDATE reserved_st SET total_reserved = total_reserved - NEW.st_units, last_updated = NOW();
        END IF;
        RETURN NEW;
    END IF;
    RETURN NULL;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_reserved_st_trigger AFTER INSERT OR UPDATE ON sales
    FOR EACH ROW EXECUTE FUNCTION update_reserved_st();

-- Create view for user balance summary
CREATE OR REPLACE VIEW user_balance_summary AS
SELECT 
    u.id as user_id,
    u.phone_hash,
    u.wallet_addr,
    u.kyc_status,
    COALESCE(st.units, 0) as st_balance,
    COALESCE(ut.units, 0) as ut_balance,
    COALESCE(ut.voting_power, 0) as voting_power,
    COALESCE(claimable.claimable_st, 0) as claimable_st,
    u.created_at,
    u.updated_at
FROM users u
LEFT JOIN st_balances st ON u.id = st.user_id
LEFT JOIN ut_balances ut ON u.id = ut.user_id
LEFT JOIN (
    SELECT 
        s.user_id,
        SUM(s.st_units) as claimable_st
    FROM sales s
    WHERE s.status = 'pending'
    GROUP BY s.user_id
) claimable ON u.id = claimable.user_id;

-- Create view for conversion round statistics
CREATE OR REPLACE VIEW conversion_round_stats AS
SELECT 
    cr.round_id,
    cr.total_pool,
    cr.total_ut_snapshot,
    cr.distributed,
    cr.status,
    cr.created_at,
    cr.completed_at,
    COUNT(ca.id) as total_allocations,
    COUNT(CASE WHEN ca.kyc_status = 'verified' THEN 1 END) as verified_allocations,
    COUNT(CASE WHEN ca.transaction_hash IS NOT NULL THEN 1 END) as completed_allocations
FROM conversion_rounds cr
LEFT JOIN conversion_allocations ca ON cr.round_id = ca.round_id
GROUP BY cr.round_id, cr.total_pool, cr.total_ut_snapshot, cr.distributed, cr.status, cr.created_at, cr.completed_at;

-- Create view for governance proposal results
CREATE OR REPLACE VIEW governance_proposal_results AS
SELECT 
    gp.proposal_id,
    gp.title,
    gp.status,
    gp.voting_start,
    gp.voting_end,
    COUNT(gv.id) as total_votes,
    SUM(CASE WHEN gv.vote = true THEN gv.voting_power ELSE 0 END) as yes_power,
    SUM(CASE WHEN gv.vote = false THEN gv.voting_power ELSE 0 END) as no_power,
    SUM(gv.voting_power) as total_voting_power
FROM governance_proposals gp
LEFT JOIN governance_votes gv ON gp.proposal_id = gv.proposal_id
GROUP BY gp.proposal_id, gp.title, gp.status, gp.voting_start, gp.voting_end;
