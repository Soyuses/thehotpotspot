//! New Tokenomics Demo for The Hot Pot Spot
//! 
//! This example demonstrates the new ST/UT tokenomics model with streaming integration.

use std::sync::Arc;
use tokio::sync::RwLock;
use blockchain_project::{
    tokenomics_config::TokenomicsConfig,
    new_tokenomics::NewTokenomicsManager,
    new_database::{NewDatabaseManager, NewDatabaseConfig},
    new_relayer_service::{NewRelayerService, RelayerConfig, SaleRequest, SaleItem},
    stream_collector::{StreamCollector, StreamCollectorConfig, ShareType},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 The Hot Pot Spot - New Tokenomics Demo");
    println!("==========================================");

    // Initialize configuration
    let tokenomics_config = TokenomicsConfig::default();
    let db_config = NewDatabaseConfig::default();
    let relayer_config = RelayerConfig::default();
    let stream_config = StreamCollectorConfig::default();

    println!("📊 Configuration:");
    println!("  - ST per GEL: {}", relayer_config.st_per_gel);
    println!("  - UT per minute: {}", stream_config.ut_per_minute);
    println!("  - UT per comment: {}", stream_config.ut_per_comment);
    println!("  - UT per share: {}", stream_config.ut_per_share);
    println!("  - Max UT per day: {}", stream_config.max_ut_per_day);

    // Initialize services
    let tokenomics_manager = Arc::new(RwLock::new(NewTokenomicsManager::new(tokenomics_config)));
    let database = Arc::new(NewDatabaseManager::new(db_config).await?);
    let relayer = NewRelayerService::new(relayer_config, tokenomics_manager.clone(), database.clone());
    let stream_collector = StreamCollector::new(stream_config, tokenomics_manager.clone(), database.clone());

    println!("\n🛒 Simulating POS Sale...");
    
    // Simulate a sale from POS system
    let sale_request = SaleRequest {
        sale_id: "sale_001".to_string(),
        node_id: "node_tbilisi_001".to_string(),
        customer_phone: Some("+995123456789".to_string()),
        amount_gel: 25.0,
        items: vec![
            SaleItem {
                item_id: "plov_001".to_string(),
                name: "Plov with Lamb".to_string(),
                price_gel: 15.0,
                quantity: 1,
            },
            SaleItem {
                item_id: "khachapuri_001".to_string(),
                name: "Khachapuri".to_string(),
                price_gel: 10.0,
                quantity: 1,
            },
        ],
        timestamp: chrono::Utc::now(),
    };

    // Process sale through relayer
    let sale_response = relayer.process_sale(sale_request).await?;
    println!("✅ Sale processed successfully!");
    println!("  - Sale ID: {}", sale_response.sale_id);
    println!("  - Check Address: {}", sale_response.check_address);
    println!("  - Activation Code: {}", sale_response.activation_code);
    println!("  - ST Units: {}", sale_response.st_units);

    println!("\n📱 Simulating Customer Check Claim...");
    
    // Simulate customer claiming ST tokens
    let user_wallet = "0x1234567890abcdef1234567890abcdef12345678".to_string();
    let minting = relayer.claim_st_tokens(
        sale_response.check_address,
        sale_response.activation_code,
        "+995123456789".to_string(),
        user_wallet.clone(),
    ).await?;

    println!("✅ ST tokens claimed successfully!");
    println!("  - Mint ID: {}", minting.mint_id);
    println!("  - Units: {}", minting.units);
    println!("  - To Address: {}", minting.to_address);
    println!("  - Transaction Hash: {}", minting.transaction_hash);

    println!("\n🎥 Simulating Streaming Activities...");
    
    let user_id = "1".to_string(); // Assuming user ID 1 from database
    
    // Start streaming session
    let session = stream_collector.start_streaming_session(
        user_id.clone(),
        "stream_001".to_string(),
        "twitch".to_string(),
    ).await?;

    println!("✅ Streaming session started!");
    println!("  - Session ID: {}", session.session_id);
    println!("  - Stream ID: {}", session.stream_id);
    println!("  - Platform: {}", session.platform);

    // Simulate streaming for 30 minutes
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await; // Simulate time passing
    
    // End streaming session
    let completed_session = stream_collector.end_streaming_session(session.session_id).await?;
    println!("✅ Streaming session completed!");
    println!("  - Duration: {} minutes", completed_session.duration_minutes);
    println!("  - UT Earned: {}", completed_session.ut_earned);

    // Simulate user interactions
    println!("\n💬 Simulating User Interactions...");
    
    // User comments
    let comment = stream_collector.record_comment(
        user_id.clone(),
        "stream_001".to_string(),
        "twitch".to_string(),
        "Great food! Love the plov!".to_string(),
    ).await?;
    println!("✅ Comment recorded! UT earned: {}", comment.ut_earned);

    // User shares
    let share = stream_collector.record_share(
        user_id.clone(),
        "stream_001".to_string(),
        "twitch".to_string(),
        ShareType::Social,
    ).await?;
    println!("✅ Share recorded! UT earned: {}", share.ut_earned);

    // User likes
    let like = stream_collector.record_like(
        user_id.clone(),
        "stream_001".to_string(),
        "twitch".to_string(),
    ).await?;
    println!("✅ Like recorded! UT earned: {}", like.ut_earned);

    // User views
    let view = stream_collector.record_view(
        user_id.clone(),
        "stream_001".to_string(),
        "twitch".to_string(),
    ).await?;
    println!("✅ View recorded! UT earned: {}", view.ut_earned);

    println!("\n📊 Getting Statistics...");
    
    // Get relayer statistics
    let relayer_stats = relayer.get_stats().await;
    println!("📈 Relayer Statistics:");
    println!("  - Total Sales: {}", relayer_stats.total_sales);
    println!("  - Successful Sales: {}", relayer_stats.successful_sales);
    println!("  - Failed Sales: {}", relayer_stats.failed_sales);
    println!("  - Total ST Minted: {}", relayer_stats.total_st_minted);
    println!("  - Total Amount (GEL): {:.2}", relayer_stats.total_amount_gel);

    // Get stream collector statistics
    let stream_stats = stream_collector.get_stats().await;
    println!("📈 Stream Collector Statistics:");
    println!("  - Total Sessions: {}", stream_stats.total_sessions);
    println!("  - Active Sessions: {}", stream_stats.active_sessions);
    println!("  - Completed Sessions: {}", stream_stats.completed_sessions);
    println!("  - Total Comments: {}", stream_stats.total_comments);
    println!("  - Total Shares: {}", stream_stats.total_shares);
    println!("  - Total Likes: {}", stream_stats.total_likes);
    println!("  - Total Views: {}", stream_stats.total_views);
    println!("  - Total UT Awarded: {}", stream_stats.total_ut_awarded);

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
    println!("  - POS sale processing and ST token minting");
    println!("  - Customer check claiming with KYC integration");
    println!("  - Streaming session management");
    println!("  - UT token awarding for user engagement");
    println!("  - Statistics collection and reporting");
    println!("  - Conversion round triggering");
    println!("\n🚀 New tokenomics model is working correctly!");
    println!("   ST tokens are minted for purchases (1 ST per 1 GEL)");
    println!("   UT tokens are awarded for streaming activities");
    println!("   Conversion rounds distribute 50% of reserved ST to UT holders");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_configuration() {
        let tokenomics_config = TokenomicsConfig::default();
        let relayer_config = RelayerConfig::default();
        let stream_config = StreamCollectorConfig::default();

        assert_eq!(relayer_config.st_per_gel, 100);
        assert_eq!(stream_config.ut_per_minute, 10);
        assert_eq!(stream_config.ut_per_comment, 5);
        assert_eq!(stream_config.ut_per_share, 20);
        assert_eq!(stream_config.ut_per_like, 2);
        assert_eq!(stream_config.ut_per_view, 1);
    }

    #[tokio::test]
    async fn test_sale_request_creation() {
        let sale_request = SaleRequest {
            sale_id: "test_sale".to_string(),
            node_id: "test_node".to_string(),
            customer_phone: Some("+995123456789".to_string()),
            amount_gel: 10.0,
            items: vec![SaleItem {
                item_id: "test_item".to_string(),
                name: "Test Item".to_string(),
                price_gel: 10.0,
                quantity: 1,
            }],
            timestamp: chrono::Utc::now(),
        };

        assert_eq!(sale_request.sale_id, "test_sale");
        assert_eq!(sale_request.amount_gel, 10.0);
        assert_eq!(sale_request.items.len(), 1);
    }

    #[tokio::test]
    async fn test_streaming_session_creation() {
        let session = blockchain_project::stream_collector::StreamingSession {
            session_id: "test_session".to_string(),
            user_id: "test_user".to_string(),
            stream_id: "test_stream".to_string(),
            platform: "twitch".to_string(),
            start_time: chrono::Utc::now(),
            end_time: None,
            duration_minutes: 0,
            ut_earned: 0,
            status: blockchain_project::stream_collector::SessionStatus::Active,
        };

        assert_eq!(session.session_id, "test_session");
        assert_eq!(session.user_id, "test_user");
        assert_eq!(session.platform, "twitch");
    }
}
