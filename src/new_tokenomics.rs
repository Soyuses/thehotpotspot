//! New Tokenomics Module for The Hot Pot Spot
//! 
//! This module implements the new tokenomics model with Security Tokens (ST) and Utility Tokens (UT)
//! according to the updated business requirements.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::tokenomics_config::{TokenomicsConfig, TOKEN_SCALE};

/// Security Token (ST) - Digital shares with dividend rights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityToken {
    /// Token ID
    pub token_id: String,
    /// Owner address
    pub owner_address: String,
    /// Token balance in subunits
    pub balance: u128,
    /// KYC status
    pub kyc_status: KycStatus,
    /// Transfer restrictions
    pub transfer_restricted: bool,
    /// Dividend eligibility
    pub dividend_eligible: bool,
    /// Last dividend snapshot
    pub last_dividend_snapshot: Option<u64>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// Utility Token (UT) - Non-transferable tokens for DAO participation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtilityToken {
    /// Token ID
    pub token_id: String,
    /// Owner address
    pub owner_address: String,
    /// Token balance in subunits
    pub balance: u128,
    /// Non-transferable flag (Soulbound)
    pub non_transferable: bool,
    /// Voting power
    pub voting_power: u128,
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// KYC status for Security Tokens
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KycStatus {
    NotRequired,
    Pending,
    Verified,
    Rejected,
    Expired,
}

/// Sale record for ST token minting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleRecord {
    /// Unique sale ID
    pub sale_id: String,
    /// Node ID where sale occurred
    pub node_id: String,
    /// User ID (if known)
    pub user_id: Option<String>,
    /// Amount in GEL
    pub amount_gel: f64,
    /// ST tokens to be minted
    pub st_units: u128,
    /// Check address (custodial)
    pub check_address: String,
    /// Activation code hash
    pub activation_code_hash: String,
    /// Sale timestamp
    pub timestamp: DateTime<Utc>,
    /// Status
    pub status: SaleStatus,
}

/// Sale status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SaleStatus {
    Pending,
    Processed,
    Claimed,
    Expired,
}

/// UT event for activity tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtEvent {
    /// Event ID
    pub event_id: String,
    /// User ID
    pub user_id: String,
    /// Event type
    pub event_type: UtEventType,
    /// UT units awarded
    pub units: u128,
    /// Reference (stream ID, comment ID, etc.)
    pub reference: String,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Platform (twitch, youtube, etc.)
    pub platform: String,
}

/// UT event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UtEventType {
    Streaming,
    Comment,
    Share,
    Like,
    View,
}

/// ST minting record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StMinting {
    /// Minting ID
    pub mint_id: String,
    /// Sale ID reference
    pub sale_id: String,
    /// Units minted
    pub units: u128,
    /// Destination address
    pub to_address: String,
    /// Transaction hash
    pub transaction_hash: String,
    /// Minting timestamp
    pub timestamp: DateTime<Utc>,
}

/// Conversion round for UT to ST distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionRound {
    /// Round ID
    pub round_id: String,
    /// Total pool size (ST)
    pub total_pool: u128,
    /// Total UT snapshot
    pub total_ut_snapshot: u128,
    /// Distributed ST
    pub distributed: u128,
    /// Round timestamp
    pub timestamp: DateTime<Utc>,
    /// Status
    pub status: ConversionRoundStatus,
}

/// Conversion round status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConversionRoundStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// Conversion allocation for individual users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionAllocation {
    /// Round ID
    pub round_id: String,
    /// User ID
    pub user_id: String,
    /// Allocated ST units
    pub allocated_units: u128,
    /// KYC status at time of allocation
    pub kyc_status: KycStatus,
    /// Transaction hash (if minted)
    pub transaction_hash: Option<String>,
    /// Allocation timestamp
    pub timestamp: DateTime<Utc>,
}

/// User balance summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBalance {
    /// User ID
    pub user_id: String,
    /// ST balance
    pub st_balance: u128,
    /// UT balance
    pub ut_balance: u128,
    /// Claimable ST (from unclaimed checks)
    pub claimable_st: u128,
    /// Total voting power
    pub voting_power: u128,
    /// KYC status
    pub kyc_status: KycStatus,
    /// Last updated
    pub last_updated: DateTime<Utc>,
}

/// New Tokenomics Manager
#[derive(Debug, Clone)]
pub struct NewTokenomicsManager {
    /// Configuration
    pub config: TokenomicsConfig,
    /// ST token holders
    pub st_holders: HashMap<String, SecurityToken>,
    /// UT token holders
    pub ut_holders: HashMap<String, UtilityToken>,
    /// Sale records
    pub sales: HashMap<String, SaleRecord>,
    /// UT events
    pub ut_events: Vec<UtEvent>,
    /// ST mintings
    pub st_mintings: Vec<StMinting>,
    /// Conversion rounds
    pub conversion_rounds: Vec<ConversionRound>,
    /// Conversion allocations
    pub conversion_allocations: Vec<ConversionAllocation>,
    /// Reserved ST (unclaimed checks, etc.)
    pub reserved_st: u128,
}

impl NewTokenomicsManager {
    /// Create a new tokenomics manager
    pub fn new(config: TokenomicsConfig) -> Self {
        Self {
            config,
            st_holders: HashMap::new(),
            ut_holders: HashMap::new(),
            sales: HashMap::new(),
            ut_events: Vec::new(),
            st_mintings: Vec::new(),
            conversion_rounds: Vec::new(),
            conversion_allocations: Vec::new(),
            reserved_st: 0,
        }
    }

    /// Record a sale and mint ST tokens
    pub fn record_sale(
        &mut self,
        sale_id: String,
        node_id: String,
        amount_gel: f64,
        user_id: Option<String>,
        check_address: String,
        activation_code_hash: String,
    ) -> Result<StMinting, String> {
        // Calculate ST tokens
        let st_units = self.config.calculate_st_tokens(amount_gel);
        
        // Create sale record
        let sale_record = SaleRecord {
            sale_id: sale_id.clone(),
            node_id,
            user_id: user_id.clone(),
            amount_gel,
            st_units,
            check_address: check_address.clone(),
            activation_code_hash,
            timestamp: Utc::now(),
            status: SaleStatus::Pending,
        };
        
        self.sales.insert(sale_id.clone(), sale_record);
        
        // Create ST minting record
        let minting = StMinting {
            mint_id: format!("MINT_{}", Utc::now().timestamp()),
            sale_id: sale_id.clone(),
            units: st_units,
            to_address: check_address,
            transaction_hash: format!("0x{}", hex::encode(&sale_id.as_bytes())),
            timestamp: Utc::now(),
        };
        
        self.st_mintings.push(minting.clone());
        
        // Update reserved ST
        self.reserved_st += st_units;
        
        Ok(minting)
    }

    /// Award UT tokens for activity
    pub fn award_ut_tokens(
        &mut self,
        user_id: String,
        event_type: UtEventType,
        reference: String,
        platform: String,
        duration_minutes: Option<u32>,
        count: Option<u32>,
    ) -> Result<u128, String> {
        let units = match event_type {
            UtEventType::Streaming => {
                if let Some(minutes) = duration_minutes {
                    self.config.calculate_ut_for_streaming(minutes)
                } else {
                    return Err("Duration required for streaming event".to_string());
                }
            }
            _ => {
                if let Some(cnt) = count {
                    let action_str = match event_type {
                        UtEventType::Comment => "comment",
                        UtEventType::Share => "share",
                        UtEventType::Like => "like",
                        _ => return Err("Invalid event type for count-based calculation".to_string()),
                    };
                    self.config.calculate_ut_for_action(action_str, cnt)
                } else {
                    return Err("Count required for action event".to_string());
                }
            }
        };

        if units == 0 {
            return Ok(0);
        }

        // Create UT event
        let event = UtEvent {
            event_id: format!("UT_EVENT_{}", Utc::now().timestamp()),
            user_id: user_id.clone(),
            event_type,
            units,
            reference,
            timestamp: Utc::now(),
            platform,
        };

        self.ut_events.push(event);

        // Update UT balance
        self.update_ut_balance(&user_id, units)?;

        Ok(units)
    }

    /// Update UT balance for user
    fn update_ut_balance(&mut self, user_id: &str, units: u128) -> Result<(), String> {
        if let Some(ut_holder) = self.ut_holders.get_mut(user_id) {
            ut_holder.balance += units;
            ut_holder.voting_power = ut_holder.balance; // 1:1 voting power
            ut_holder.last_activity = Utc::now();
            ut_holder.updated_at = Utc::now();
        } else {
            // Create new UT holder
            let ut_holder = UtilityToken {
                token_id: format!("UT_{}", user_id),
                owner_address: user_id.to_string(),
                balance: units,
                non_transferable: self.config.utility_token.non_transferable,
                voting_power: units,
                last_activity: Utc::now(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            self.ut_holders.insert(user_id.to_string(), ut_holder);
        }

        Ok(())
    }

    /// Claim ST tokens from check
    pub fn claim_st_tokens(
        &mut self,
        check_address: String,
        activation_code: String,
        user_wallet: String,
        kyc_status: KycStatus,
    ) -> Result<StMinting, String> {
        // Find sale record by check address
        let sale_id = self.sales.values()
            .find(|sale| sale.check_address == check_address)
            .map(|sale| sale.sale_id.clone())
            .ok_or("Sale not found")?;

        // Get sale record mutably
        let sale_record = self.sales.get_mut(&sale_id)
            .ok_or("Sale not found")?;

        if sale_record.status != SaleStatus::Pending {
            return Err("Sale already processed".to_string());
        }

        // Verify activation code (simplified)
        if sale_record.activation_code_hash != format!("{:x}", md5::compute(activation_code)) {
            return Err("Invalid activation code".to_string());
        }

        let st_units = sale_record.st_units;
        let sale_id_clone = sale_record.sale_id.clone();

        // Update sale status
        sale_record.status = SaleStatus::Claimed;

        // Create ST token for user
        let st_token = SecurityToken {
            token_id: format!("ST_{}", user_wallet),
            owner_address: user_wallet.clone(),
            balance: st_units,
            kyc_status: kyc_status.clone(),
            transfer_restricted: kyc_status != KycStatus::Verified,
            dividend_eligible: true,
            last_dividend_snapshot: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.st_holders.insert(user_wallet.clone(), st_token);

        // Update reserved ST
        self.reserved_st = self.reserved_st.saturating_sub(st_units);

        // Create minting record
        let minting = StMinting {
            mint_id: format!("CLAIM_{}", Utc::now().timestamp()),
            sale_id: sale_id_clone.clone(),
            units: st_units,
            to_address: user_wallet,
            transaction_hash: format!("0x{}", hex::encode(&sale_id_clone.as_bytes())),
            timestamp: Utc::now(),
        };

        self.st_mintings.push(minting.clone());

        Ok(minting)
    }

    /// Trigger conversion round
    pub fn trigger_conversion_round(&mut self) -> Result<ConversionRound, String> {
        let pool_size = self.config.get_conversion_pool_size(self.reserved_st);
        
        if pool_size == 0 {
            return Err("No reserved ST available for conversion".to_string());
        }

        // Get UT snapshot for distribution
        let ut_snapshot = self.get_total_ut_snapshot();
        if ut_snapshot == 0 {
            return Err("No UT holders available for conversion".to_string());
        }

        // Create conversion round
        let round = ConversionRound {
            round_id: format!("ROUND_{}", Utc::now().timestamp()),
            total_pool: pool_size,
            total_ut_snapshot: ut_snapshot,
            distributed: 0,
            timestamp: Utc::now(),
            status: ConversionRoundStatus::InProgress,
        };

        self.conversion_rounds.push(round.clone());

        // Distribute pool to UT holders
        let distributed_amount = self.distribute_conversion_pool(&round)?;
        
        // Update round status
        if let Some(last_round) = self.conversion_rounds.last_mut() {
            last_round.distributed = distributed_amount;
            last_round.status = ConversionRoundStatus::Completed;
        }

        // Reduce reserved ST by distributed amount
        self.reserved_st = self.reserved_st.saturating_sub(distributed_amount);

        Ok(round)
    }

    /// Distribute conversion pool to UT holders
    fn distribute_conversion_pool(&mut self, round: &ConversionRound) -> Result<u128, String> {
        if round.total_ut_snapshot == 0 {
            return Ok(0);
        }

        let mut total_distributed = 0u128;

        for (user_id, ut_holder) in &self.ut_holders {
            if !self.config.can_participate_in_conversion(ut_holder.balance) {
                continue;
            }

            // Calculate allocation proportional to UT balance
            let allocation = (round.total_pool * ut_holder.balance) / round.total_ut_snapshot;
            
            if allocation == 0 {
                continue;
            }

            // Check anti-whale cap
            let capped_allocation = std::cmp::min(
                allocation,
                self.config.conversion_pool.max_st_per_user_per_round
            );

            // Create allocation record
            let allocation_record = ConversionAllocation {
                round_id: round.round_id.clone(),
                user_id: user_id.clone(),
                allocated_units: capped_allocation,
                kyc_status: self.get_user_kyc_status(user_id),
                transaction_hash: None,
                timestamp: Utc::now(),
            };

            self.conversion_allocations.push(allocation_record);
            total_distributed += capped_allocation;
        }

        Ok(total_distributed)
    }

    /// Get total UT snapshot
    fn get_total_ut_snapshot(&self) -> u128 {
        self.ut_holders.values()
            .filter(|holder| self.config.can_participate_in_conversion(holder.balance))
            .map(|holder| holder.balance)
            .sum()
    }

    /// Get conversion allocations for a specific round
    pub fn get_conversion_allocations(&self, round_id: &str) -> Vec<&ConversionAllocation> {
        self.conversion_allocations
            .iter()
            .filter(|allocation| allocation.round_id == round_id)
            .collect()
    }

    /// Get user's conversion history
    pub fn get_user_conversion_history(&self, user_id: &str) -> Vec<&ConversionAllocation> {
        self.conversion_allocations
            .iter()
            .filter(|allocation| allocation.user_id == user_id)
            .collect()
    }

    /// Get total ST allocated to user across all rounds
    pub fn get_user_total_allocated_st(&self, user_id: &str) -> u128 {
        self.conversion_allocations
            .iter()
            .filter(|allocation| allocation.user_id == user_id)
            .map(|allocation| allocation.allocated_units)
            .sum()
    }

    /// Get user KYC status
    fn get_user_kyc_status(&self, user_id: &str) -> KycStatus {
        self.st_holders.get(user_id)
            .map(|st| st.kyc_status.clone())
            .unwrap_or(KycStatus::NotRequired)
    }

    /// Get user balance summary
    pub fn get_user_balance(&self, user_id: &str) -> UserBalance {
        let st_balance = self.st_holders.get(user_id)
            .map(|st| st.balance)
            .unwrap_or(0);
        
        let ut_balance = self.ut_holders.get(user_id)
            .map(|ut| ut.balance)
            .unwrap_or(0);

        let claimable_st = self.sales.values()
            .filter(|sale| sale.user_id.as_ref() == Some(&user_id.to_string()))
            .filter(|sale| sale.status == SaleStatus::Pending)
            .map(|sale| sale.st_units)
            .sum();

        let voting_power = self.ut_holders.get(user_id)
            .map(|ut| ut.voting_power)
            .unwrap_or(0);

        let kyc_status = self.get_user_kyc_status(user_id);

        UserBalance {
            user_id: user_id.to_string(),
            st_balance,
            ut_balance,
            claimable_st,
            voting_power,
            kyc_status,
            last_updated: Utc::now(),
        }
    }

    /// Check if user can create governance proposals
    pub fn can_create_proposal(&self, user_id: &str) -> bool {
        self.ut_holders.get(user_id)
            .map(|ut| self.config.can_create_proposal(ut.balance))
            .unwrap_or(false)
    }

    /// Check if user can vote in governance
    pub fn can_vote(&self, user_id: &str) -> bool {
        self.ut_holders.get(user_id)
            .map(|ut| self.config.can_vote(ut.balance))
            .unwrap_or(false)
    }

    /// Get conversion round statistics
    pub fn get_conversion_stats(&self) -> ConversionStats {
        ConversionStats {
            total_st_holders: self.st_holders.len() as u32,
            total_ut_holders: self.ut_holders.len() as u32,
            total_sales: self.sales.len() as u32,
            total_ut_events: self.ut_events.len() as u32,
            total_st_minted: self.st_mintings.iter().map(|m| m.units).sum(),
            total_ut_awarded: self.ut_events.iter().map(|e| e.units).sum(),
            total_rounds: self.conversion_rounds.len(),
            reserved_st: self.reserved_st,
        }
    }

    /// Add sale record
    pub fn add_sale(&mut self, sale: SaleRecord) -> Result<(), String> {
        if self.sales.contains_key(&sale.sale_id) {
            return Err("Sale already exists".to_string());
        }

        // Update reserved ST
        self.reserved_st += sale.st_units;
        
        // Add sale record
        self.sales.insert(sale.sale_id.clone(), sale.clone());
        
        // Mint ST tokens to user if user_id is provided
        if let Some(user_id) = &sale.user_id {
            let st_holder = self.st_holders.entry(user_id.clone()).or_insert(SecurityToken {
                token_id: format!("st_{}", user_id),
                owner_address: user_id.clone(),
                balance: 0,
                kyc_status: KycStatus::NotRequired,
                transfer_restricted: true,
                dividend_eligible: false,
                last_dividend_snapshot: None,
                created_at: sale.timestamp,
                updated_at: sale.timestamp,
            });
            
            st_holder.balance += sale.st_units;
            st_holder.updated_at = sale.timestamp;
            
            // Record minting
            self.st_mintings.push(StMinting {
                mint_id: format!("mint_{}", sale.sale_id),
                sale_id: sale.sale_id.clone(),
                units: sale.st_units,
                to_address: user_id.clone(),
                transaction_hash: format!("0x{}", hex::encode(sale.sale_id.as_bytes())),
                timestamp: sale.timestamp,
            });
        }
        
        Ok(())
    }

    /// Add UT event
    pub fn add_ut_event(&mut self, event: UtEvent) -> Result<(), String> {
        // Add event to the list
        self.ut_events.push(event.clone());
        
        // Update user's UT balance
        let user_id = &event.user_id;
        let ut_holder = self.ut_holders.entry(user_id.clone()).or_insert(UtilityToken {
            token_id: format!("ut_{}", user_id),
            owner_address: user_id.clone(),
            balance: 0,
            non_transferable: true,
            voting_power: 0,
            last_activity: event.timestamp,
            created_at: event.timestamp,
            updated_at: event.timestamp,
        });
        
        ut_holder.balance += event.units;
        ut_holder.voting_power += event.units;
        ut_holder.last_activity = event.timestamp;
        ut_holder.updated_at = event.timestamp;
        
        Ok(())
    }

    /// Get UT balance for a user
    pub fn get_ut_balance(&self, user_id: &str) -> u128 {
        self.ut_holders.get(user_id).map(|holder| holder.balance).unwrap_or(0)
    }
    
    /// Get total UT supply
    pub fn get_total_ut_supply(&self) -> u128 {
        self.ut_holders.values().map(|holder| holder.balance).sum()
    }
    
    /// Deduct UT from user (for proposal deposits, etc.)
    pub fn deduct_ut(&mut self, user_id: &str, amount: u128) -> Result<(), String> {
        if let Some(ut_holder) = self.ut_holders.get_mut(user_id) {
            if ut_holder.balance >= amount {
                ut_holder.balance -= amount;
                Ok(())
            } else {
                Err("Insufficient UT balance".to_string())
            }
        } else {
            Err("User not found".to_string())
        }
    }

    /// Get statistics
    pub fn get_statistics(&self) -> Result<ConversionStats, String> {
        Ok(ConversionStats {
            total_st_holders: self.st_holders.len() as u32,
            total_ut_holders: self.ut_holders.len() as u32,
            total_sales: self.sales.len() as u32,
            total_ut_events: self.ut_events.len() as u32,
            total_st_minted: self.st_mintings.iter().map(|m| m.units).sum(),
            total_ut_awarded: self.ut_events.iter().map(|e| e.units).sum(),
            total_rounds: self.conversion_rounds.len(),
            reserved_st: self.reserved_st,
        })
    }
}

/// Conversion statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionStats {
    pub total_st_holders: u32,
    pub total_ut_holders: u32,
    pub total_sales: u32,
    pub total_ut_events: u32,
    pub total_st_minted: u128,
    pub total_ut_awarded: u128,
    pub total_rounds: usize,
    pub reserved_st: u128,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tokenomics_manager() {
        let config = TokenomicsConfig::default();
        let manager = NewTokenomicsManager::new(config);
        assert_eq!(manager.reserved_st, 0);
        assert!(manager.st_holders.is_empty());
        assert!(manager.ut_holders.is_empty());
    }

    #[test]
    fn test_record_sale() {
        let config = TokenomicsConfig::default();
        let mut manager = NewTokenomicsManager::new(config);
        
        let result = manager.record_sale(
            "SALE_001".to_string(),
            "NODE_001".to_string(),
            10.0, // 10 GEL
            Some("USER_001".to_string()),
            "CHECK_ADDR_001".to_string(),
            "ACTIVATION_HASH".to_string(),
        );
        
        assert!(result.is_ok());
        assert_eq!(manager.reserved_st, 1000); // 10 GEL * 100 ST per GEL
        assert!(manager.sales.contains_key("SALE_001"));
    }

    #[test]
    fn test_award_ut_tokens() {
        let config = TokenomicsConfig::default();
        let mut manager = NewTokenomicsManager::new(config);
        
        let result = manager.award_ut_tokens(
            "USER_001".to_string(),
            UtEventType::Streaming,
            "STREAM_001".to_string(),
            "twitch".to_string(),
            Some(10), // 10 minutes
            None,
        );
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100); // 10 minutes * 10 UT per minute
        assert!(manager.ut_holders.contains_key("USER_001"));
    }

    #[test]
    fn test_claim_st_tokens() {
        let config = TokenomicsConfig::default();
        let mut manager = NewTokenomicsManager::new(config);
        
        // First record a sale
        let _ = manager.record_sale(
            "SALE_001".to_string(),
            "NODE_001".to_string(),
            10.0,
            Some("USER_001".to_string()),
            "CHECK_ADDR_001".to_string(),
            format!("{:x}", md5::compute("123456")),
        );
        
        // Then claim the tokens
        let result = manager.claim_st_tokens(
            "CHECK_ADDR_001".to_string(),
            "123456".to_string(),
            "USER_WALLET_001".to_string(),
            KycStatus::Verified,
        );
        
        assert!(result.is_ok());
        assert!(manager.st_holders.contains_key("USER_WALLET_001"));
        assert_eq!(manager.reserved_st, 0);
    }

    #[test]
    fn test_conversion_round() {
        let config = TokenomicsConfig::default();
        let mut manager = NewTokenomicsManager::new(config);
        
        // Set up some reserved ST
        manager.reserved_st = 10000;
        
        // Award some UT tokens
        let _ = manager.award_ut_tokens(
            "USER_001".to_string(),
            UtEventType::Streaming,
            "STREAM_001".to_string(),
            "twitch".to_string(),
            Some(100), // 100 minutes
            None,
        );
        
        // Trigger conversion round
        let result = manager.trigger_conversion_round();
        assert!(result.is_ok());
        
        let round = result.unwrap();
        assert_eq!(round.total_pool, 5000); // 50% of 10000
        assert_eq!(round.total_ut_snapshot, 1000); // 100 minutes * 10 UT per minute
    }
}
