//! Tokenomics Configuration Module for The Hot Pot Spot
//! 
//! This module defines the new tokenomics model with Security Tokens (ST) and Utility Tokens (UT)
//! according to the updated business requirements.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Token scale for precision (100 = 2 decimal places)
pub const TOKEN_SCALE: u128 = 100;

/// Security Token (ST) configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityTokenConfig {
    /// THP tokens per 1 GEL spent (1 GEL = 0.2 THP)
    pub thp_per_gel: u128,
    /// Minimum ST amount for KYC requirement
    pub min_st_for_kyc: u128,
    /// Maximum ST per transaction
    pub max_st_per_transaction: u128,
    /// KYC required for transfers
    pub kyc_required_for_transfer: bool,
    /// Pause functionality enabled
    pub pause_enabled: bool,
}

/// Utility Token (UT) configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtilityTokenConfig {
    /// UT tokens per minute of streaming
    pub ut_per_minute: u128,
    /// UT tokens per comment
    pub ut_per_comment: u128,
    /// UT tokens per share/repost
    pub ut_per_share: u128,
    /// UT tokens per like
    pub ut_per_like: u128,
    /// Maximum UT per day per user
    pub max_ut_per_day: u128,
    /// Minimum streaming time for UT accrual (in minutes)
    pub min_streaming_time_minutes: u32,
    /// UT are non-transferable (Soulbound)
    pub non_transferable: bool,
}

/// Conversion pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionPoolConfig {
    /// Percentage of reserved ST to distribute (50%)
    pub conversion_pool_share: u8,
    /// Minimum UT balance for participation
    pub min_ut_for_participation: u128,
    /// Maximum ST per user per round (anti-whale)
    pub max_st_per_user_per_round: u128,
    /// Conversion rounds frequency (in days)
    pub conversion_frequency_days: u32,
    /// Last conversion round timestamp
    pub last_conversion_timestamp: u64,
}

/// Governance/DAO configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceConfig {
    /// Minimum UT required to create proposals
    pub min_ut_for_proposal: u128,
    /// Minimum UT required to vote
    pub min_ut_for_voting: u128,
    /// Voting period duration (in hours)
    pub voting_period_hours: u32,
    /// Quorum percentage required
    pub quorum_percentage: u8,
    /// Execution delay after vote passes (in hours)
    pub execution_delay_hours: u32,
}

/// Streaming integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingConfig {
    /// Supported streaming platforms
    pub supported_platforms: Vec<String>,
    /// Webhook endpoints for platform events
    pub webhook_endpoints: HashMap<String, String>,
    /// Anti-bot detection settings
    pub anti_bot_settings: AntiBotConfig,
    /// Viewer tracking settings
    pub viewer_tracking: ViewerTrackingConfig,
}

/// Anti-bot detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiBotConfig {
    /// Minimum view time for UT accrual (in seconds)
    pub min_view_time_seconds: u32,
    /// Maximum views per IP per hour
    pub max_views_per_ip_per_hour: u32,
    /// Captcha required for comments
    pub captcha_required_for_comments: bool,
    /// Social signature validation for shares
    pub social_signature_validation: bool,
}

/// Viewer tracking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewerTrackingConfig {
    /// Tracking interval (in seconds)
    pub tracking_interval_seconds: u32,
    /// Session timeout (in minutes)
    pub session_timeout_minutes: u32,
    /// Maximum concurrent viewers per stream
    pub max_concurrent_viewers: u32,
    /// Enable real-time UT accrual display
    pub real_time_ut_display: bool,
}

/// KYC/AML configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KycAmlConfig {
    /// KYC provider settings
    pub kyc_provider: String,
    /// Required KYC level for ST transfers
    pub required_kyc_level: String,
    /// AML check frequency (in days)
    pub aml_check_frequency_days: u32,
    /// Sanctions list update frequency (in days)
    pub sanctions_update_frequency_days: u32,
}

/// Main tokenomics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenomicsConfig {
    /// Security Token configuration
    pub security_token: SecurityTokenConfig,
    /// Utility Token configuration
    pub utility_token: UtilityTokenConfig,
    /// Conversion pool configuration
    pub conversion_pool: ConversionPoolConfig,
    /// Governance configuration
    pub governance: GovernanceConfig,
    /// Streaming configuration
    pub streaming: StreamingConfig,
    /// KYC/AML configuration
    pub kyc_aml: KycAmlConfig,
    /// Network configuration
    pub network: NetworkConfig,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Blockchain network type (Substrate/CosmWasm/EVM)
    pub blockchain_type: String,
    /// Network RPC endpoint
    pub rpc_endpoint: String,
    /// Relayer wallet address
    pub relayer_address: String,
    /// Master owner wallet address
    pub master_owner_address: String,
    /// Charity fund wallet address
    pub charity_fund_address: String,
    /// Gas price (in wei/gas units)
    pub gas_price: u128,
    /// Gas limit for transactions
    pub gas_limit: u128,
}

impl Default for TokenomicsConfig {
    fn default() -> Self {
        Self {
            security_token: SecurityTokenConfig {
                thp_per_gel: 20, // 1 GEL = 0.2 THP (20 units with TOKEN_SCALE 100)
                min_st_for_kyc: 1000 * TOKEN_SCALE, // 1000 ST minimum for KYC
                max_st_per_transaction: 100000 * TOKEN_SCALE, // 100,000 ST max per transaction
                kyc_required_for_transfer: true,
                pause_enabled: true,
            },
            utility_token: UtilityTokenConfig {
                ut_per_minute: 10, // 10 UT per minute of streaming
                ut_per_comment: 5, // 5 UT per comment
                ut_per_share: 20, // 20 UT per share/repost
                ut_per_like: 1, // 1 UT per like
                max_ut_per_day: 1000, // 1000 UT max per day
                min_streaming_time_minutes: 5, // Minimum 5 minutes for UT accrual
                non_transferable: true, // UT are soulbound/non-transferable
            },
            conversion_pool: ConversionPoolConfig {
                conversion_pool_share: 50, // 50% of reserved ST
                min_ut_for_participation: 100, // 100 UT minimum for participation
                max_st_per_user_per_round: 10000 * TOKEN_SCALE, // 10,000 ST max per user per round
                conversion_frequency_days: 90, // Quarterly conversion rounds
                last_conversion_timestamp: 0,
            },
            governance: GovernanceConfig {
                min_ut_for_proposal: 1000, // 1000 UT minimum for proposals
                min_ut_for_voting: 100, // 100 UT minimum for voting
                voting_period_hours: 72, // 3 days voting period
                quorum_percentage: 25, // 25% quorum required
                execution_delay_hours: 24, // 24 hours execution delay
            },
            streaming: StreamingConfig {
                supported_platforms: vec![
                    "twitch".to_string(),
                    "youtube".to_string(),
                    "rtmp".to_string(),
                ],
                webhook_endpoints: HashMap::new(),
                anti_bot_settings: AntiBotConfig {
                    min_view_time_seconds: 30, // 30 seconds minimum view time
                    max_views_per_ip_per_hour: 10, // 10 views per IP per hour
                    captcha_required_for_comments: true,
                    social_signature_validation: true,
                },
                viewer_tracking: ViewerTrackingConfig {
                    tracking_interval_seconds: 60, // Track every minute
                    session_timeout_minutes: 30, // 30 minutes session timeout
                    max_concurrent_viewers: 1000, // 1000 max concurrent viewers
                    real_time_ut_display: true,
                },
            },
            kyc_aml: KycAmlConfig {
                kyc_provider: "jumio".to_string(),
                required_kyc_level: "enhanced".to_string(),
                aml_check_frequency_days: 30,
                sanctions_update_frequency_days: 7,
            },
            network: NetworkConfig {
                blockchain_type: "substrate".to_string(),
                rpc_endpoint: "ws://localhost:9944".to_string(),
                relayer_address: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                master_owner_address: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
                charity_fund_address: "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy".to_string(),
                gas_price: 1000000000, // 1 Gwei
                gas_limit: 1000000, // 1M gas limit
            },
        }
    }
}

impl TokenomicsConfig {
    /// Create a new tokenomics configuration with custom settings
    pub fn new(
        thp_per_gel: u128,
        ut_per_minute: u128,
        conversion_pool_share: u8,
    ) -> Self {
        let mut config = Self::default();
        config.security_token.thp_per_gel = thp_per_gel;
        config.utility_token.ut_per_minute = ut_per_minute;
        config.conversion_pool.conversion_pool_share = conversion_pool_share;
        config
    }

    /// Calculate ST tokens for a given GEL amount
    pub fn calculate_st_tokens(&self, gel_amount: f64) -> u128 {
        let gel_subunits = (gel_amount * TOKEN_SCALE as f64) as u128;
        (gel_subunits * self.security_token.thp_per_gel) / TOKEN_SCALE
    }

    /// Calculate UT tokens for streaming time
    pub fn calculate_ut_for_streaming(&self, minutes: u32) -> u128 {
        if minutes < self.utility_token.min_streaming_time_minutes {
            return 0;
        }
        (minutes as u128) * self.utility_token.ut_per_minute
    }

    /// Calculate UT tokens for social actions
    pub fn calculate_ut_for_action(&self, action: &str, count: u32) -> u128 {
        let ut_per_action = match action {
            "comment" => self.utility_token.ut_per_comment,
            "share" => self.utility_token.ut_per_share,
            "like" => self.utility_token.ut_per_like,
            _ => 0,
        };
        (count as u128) * ut_per_action
    }

    /// Check if user can participate in conversion round
    pub fn can_participate_in_conversion(&self, ut_balance: u128) -> bool {
        ut_balance >= self.conversion_pool.min_ut_for_participation
    }

    /// Check if user can create governance proposals
    pub fn can_create_proposal(&self, ut_balance: u128) -> bool {
        ut_balance >= self.governance.min_ut_for_proposal
    }

    /// Check if user can vote in governance
    pub fn can_vote(&self, ut_balance: u128) -> bool {
        ut_balance >= self.governance.min_ut_for_voting
    }

    /// Get conversion pool size for given reserved ST
    pub fn get_conversion_pool_size(&self, reserved_st: u128) -> u128 {
        (reserved_st * self.conversion_pool.conversion_pool_share as u128) / 100
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.security_token.thp_per_gel == 0 {
            return Err("ST per GEL cannot be zero".to_string());
        }
        if self.utility_token.ut_per_minute == 0 {
            return Err("UT per minute cannot be zero".to_string());
        }
        if self.conversion_pool.conversion_pool_share > 100 {
            return Err("Conversion pool share cannot exceed 100%".to_string());
        }
        if self.governance.quorum_percentage > 100 {
            return Err("Quorum percentage cannot exceed 100%".to_string());
        }
        Ok(())
    }
}

/// Tokenomics events for monitoring and logging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenomicsEvent {
    /// ST tokens minted
    StMinted {
        to_address: String,
        amount: u128,
        sale_id: String,
        transaction_hash: String,
    },
    /// UT tokens awarded
    UtAwarded {
        to_address: String,
        amount: u128,
        event_type: String,
        reference: String,
    },
    /// Conversion round started
    ConversionRoundStarted {
        round_id: String,
        pool_size: u128,
        total_ut_snapshot: u128,
    },
    /// Conversion round completed
    ConversionRoundCompleted {
        round_id: String,
        distributed_st: u128,
        participants: u32,
    },
    /// Governance proposal created
    ProposalCreated {
        proposal_id: String,
        proposer: String,
        ut_balance: u128,
    },
    /// Governance vote cast
    VoteCast {
        proposal_id: String,
        voter: String,
        ut_balance: u128,
        vote: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = TokenomicsConfig::default();
        assert_eq!(config.security_token.thp_per_gel, 20);
        assert_eq!(config.utility_token.ut_per_minute, 10);
        assert_eq!(config.conversion_pool.conversion_pool_share, 50);
    }

    #[test]
    fn test_calculate_st_tokens() {
        let config = TokenomicsConfig::default();
        let st_tokens = config.calculate_st_tokens(10.0); // 10 GEL
        assert_eq!(st_tokens, 1000); // 10 * 100 = 1000 ST
    }

    #[test]
    fn test_calculate_ut_for_streaming() {
        let config = TokenomicsConfig::default();
        let ut_tokens = config.calculate_ut_for_streaming(10); // 10 minutes
        assert_eq!(ut_tokens, 100); // 10 * 10 = 100 UT
    }

    #[test]
    fn test_calculate_ut_for_streaming_minimum() {
        let config = TokenomicsConfig::default();
        let ut_tokens = config.calculate_ut_for_streaming(3); // 3 minutes (below minimum)
        assert_eq!(ut_tokens, 0); // Should be 0 due to minimum time requirement
    }

    #[test]
    fn test_calculate_ut_for_actions() {
        let config = TokenomicsConfig::default();
        let ut_tokens = config.calculate_ut_for_action("comment", 5);
        assert_eq!(ut_tokens, 25); // 5 * 5 = 25 UT
    }

    #[test]
    fn test_conversion_pool_calculation() {
        let config = TokenomicsConfig::default();
        let pool_size = config.get_conversion_pool_size(10000);
        assert_eq!(pool_size, 5000); // 50% of 10000 = 5000
    }

    #[test]
    fn test_config_validation() {
        let config = TokenomicsConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_invalid() {
        let mut config = TokenomicsConfig::default();
        config.security_token.thp_per_gel = 0;
        assert!(config.validate().is_err());
    }
}
