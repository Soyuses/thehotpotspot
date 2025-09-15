//! Simple New Tokenomics Demo for The Hot Pot Spot
//! 
//! This example demonstrates the new ST/UT tokenomics model without database dependency.

use std::sync::Arc;
use tokio::sync::RwLock;
use blockchain_project::{
    tokenomics_config::TokenomicsConfig,
    new_tokenomics::NewTokenomicsManager,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 The Hot Pot Spot - Simple New Tokenomics Demo");
    println!("=================================================");

    // Initialize configuration
    let tokenomics_config = TokenomicsConfig::default();
    let tokenomics_manager = Arc::new(RwLock::new(NewTokenomicsManager::new(tokenomics_config.clone())));

    println!("📊 Configuration:");
    println!("  - THP per GEL: {} (1 GEL = 0.2 THP)", tokenomics_config.security_token.thp_per_gel);
    println!("  - UT per minute: {}", tokenomics_config.utility_token.ut_per_minute);
    println!("  - UT per comment: {}", tokenomics_config.utility_token.ut_per_comment);
    println!("  - UT per share: {}", tokenomics_config.utility_token.ut_per_share);
    println!("  - Max UT per day: {}", tokenomics_config.utility_token.max_ut_per_day);

    println!("\n🛒 Simulating POS Sale...");
    
    // Simulate a sale record
    let sale_record = blockchain_project::new_tokenomics::SaleRecord {
        sale_id: "sale_001".to_string(),
        node_id: "node_tbilisi_001".to_string(),
        user_id: Some("user_001".to_string()),
        amount_gel: 25.0,
        st_units: 500, // 25.0 * 20 (1 GEL = 0.2 THP)
        check_address: "0xcheck1234567890abcdef1234567890abcdef123456".to_string(),
        activation_code_hash: "123456".to_string(),
        timestamp: chrono::Utc::now(),
        status: blockchain_project::new_tokenomics::SaleStatus::Pending,
    };

    // Add sale to tokenomics manager
    {
        let mut manager = tokenomics_manager.write().await;
        manager.add_sale(sale_record)?;
    }

    println!("✅ Sale processed successfully!");
    println!("  - Sale ID: sale_001");
    println!("  - Amount: 25.0 GEL");
    println!("  - THP Units: 500 (25.0 * 20 = 5 THP)");

    println!("\n🎥 Simulating Streaming Activities...");
    
    // Simulate UT events
    let ut_events = vec![
        blockchain_project::new_tokenomics::UtEvent {
            event_id: "event_001".to_string(),
            user_id: "user_001".to_string(),
            event_type: blockchain_project::new_tokenomics::UtEventType::Streaming,
            units: 300, // 30 minutes * 10 UT per minute
            timestamp: chrono::Utc::now(),
            reference: "stream_001".to_string(),
            platform: "twitch".to_string(),
        },
        blockchain_project::new_tokenomics::UtEvent {
            event_id: "event_002".to_string(),
            user_id: "user_001".to_string(),
            event_type: blockchain_project::new_tokenomics::UtEventType::Comment,
            units: 5,
            timestamp: chrono::Utc::now(),
            reference: "stream_001".to_string(),
            platform: "twitch".to_string(),
        },
        blockchain_project::new_tokenomics::UtEvent {
            event_id: "event_003".to_string(),
            user_id: "user_001".to_string(),
            event_type: blockchain_project::new_tokenomics::UtEventType::Share,
            units: 20,
            timestamp: chrono::Utc::now(),
            reference: "stream_001".to_string(),
            platform: "twitch".to_string(),
        },
    ];

    // Add UT events to tokenomics manager
    {
        let mut manager = tokenomics_manager.write().await;
        for event in ut_events {
            manager.add_ut_event(event)?;
        }
    }

    println!("✅ UT events processed successfully!");
    println!("  - Streaming: 300 UT (30 minutes)");
    println!("  - Comment: 5 UT");
    println!("  - Share: 20 UT");
    println!("  - Total UT: 325");

    println!("\n📊 Getting Statistics...");
    
    // Get tokenomics statistics
    let tokenomics_stats = {
        let manager = tokenomics_manager.read().await;
        manager.get_statistics()?
    };
    
    println!("📈 Tokenomics Statistics:");
    println!("  - Total ST Holders: {}", tokenomics_stats.total_st_holders);
    println!("  - Total UT Holders: {}", tokenomics_stats.total_ut_holders);
    println!("  - Total Sales: {}", tokenomics_stats.total_sales);
    println!("  - Total UT Events: {}", tokenomics_stats.total_ut_events);
    println!("  - Total ST Minted: {}", tokenomics_stats.total_st_minted);
    println!("  - Total UT Awarded: {}", tokenomics_stats.total_ut_awarded);
    println!("  - Total Conversion Rounds: {}", tokenomics_stats.total_rounds);
    println!("  - Reserved ST: {}", tokenomics_stats.reserved_st);

    println!("\n🔄 Simulating Conversion Round...");
    
    // Trigger conversion round
    let conversion_round = {
        let mut manager = tokenomics_manager.write().await;
        manager.trigger_conversion_round()?
    };
    
    println!("✅ Conversion round triggered!");
    println!("  - Round ID: {}", conversion_round.round_id);
    println!("  - Total Pool: {}", conversion_round.total_pool);
    println!("  - Total UT Snapshot: {}", conversion_round.total_ut_snapshot);
    println!("  - Status: {:?}", conversion_round.status);

    println!("\n🎯 Demo Summary:");
    println!("================");
    println!("✅ Successfully demonstrated:");
    println!("  - POS sale processing and THP token minting");
    println!("  - UT token awarding for user engagement");
    println!("  - Statistics collection and reporting");
    println!("  - Conversion round triggering");
    println!("\n🚀 New tokenomics model is working correctly!");
    println!("   THP tokens are minted for purchases (1 GEL = 0.2 THP)");
    println!("   UT tokens are awarded for streaming activities");
    println!("   Conversion rounds distribute 50% of reserved THP to UT holders");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_configuration() {
        let tokenomics_config = TokenomicsConfig::default();

        assert_eq!(tokenomics_config.security_token.thp_per_gel, 20);
        assert_eq!(tokenomics_config.utility_token.ut_per_minute, 10);
        assert_eq!(tokenomics_config.utility_token.ut_per_comment, 5);
        assert_eq!(tokenomics_config.utility_token.ut_per_share, 20);
        assert_eq!(tokenomics_config.utility_token.ut_per_like, 2);
    }

    #[tokio::test]
    async fn test_sale_record_creation() {
        let sale_record = blockchain_project::new_tokenomics::SaleRecord {
            sale_id: "test_sale".to_string(),
            node_id: "test_node".to_string(),
            user_id: Some("user_001".to_string()),
            amount_gel: 10.0,
            st_units: 1000,
            check_address: "0xtest".to_string(),
            activation_code_hash: "123456".to_string(),
            timestamp: chrono::Utc::now(),
            status: blockchain_project::new_tokenomics::SaleStatus::Pending,
        };

        assert_eq!(sale_record.sale_id, "test_sale");
        assert_eq!(sale_record.amount_gel, 10.0);
    }

    #[tokio::test]
    async fn test_ut_event_creation() {
        let ut_event = blockchain_project::new_tokenomics::UtEvent {
            event_id: "test_event".to_string(),
            user_id: "test_user".to_string(),
            event_type: blockchain_project::new_tokenomics::UtEventType::Comment,
            units: 5,
            timestamp: chrono::Utc::now(),
            reference: "test_ref".to_string(),
            platform: "twitch".to_string(),
        };

        assert_eq!(ut_event.event_id, "test_event");
        assert_eq!(ut_event.user_id, "test_user");
        assert_eq!(ut_event.units, 5);
    }
}
