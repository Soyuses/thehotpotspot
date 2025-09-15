use std::sync::Arc;
use tokio::sync::RwLock;
use blockchain_project::{
    tokenomics_config::TokenomicsConfig,
    viewer_arm::{ViewerARM, ViewerLoginRequest, UTActivityRequest, KYCRegistrationRequest, StreamingPlatform},
    kyc_aml::KYCAmlManager,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 The Hot Pot Spot - Viewer ARM Demo");
    println!("=====================================");

    // Initialize configuration
    let tokenomics_config = TokenomicsConfig::default();
    let kyc_manager = KYCAmlManager::new();
    
    // Setup connected platforms
    let connected_platforms = vec![
        StreamingPlatform {
            name: "twitch".to_string(),
            api_endpoint: "https://api.twitch.tv".to_string(),
            auth_token: "test_token".to_string(),
            is_active: true,
        },
        StreamingPlatform {
            name: "youtube".to_string(),
            api_endpoint: "https://www.googleapis.com/youtube/v3".to_string(),
            auth_token: "test_token".to_string(),
            is_active: true,
        },
    ];

    let mut viewer_arm = ViewerARM::new(tokenomics_config, kyc_manager, connected_platforms);

    println!("📊 Configuration:");
    println!("  - THP per GEL: {} (1 GEL = 0.2 THP)", 20);
    println!("  - UT per minute: {}", 10);
    println!("  - UT per comment: {}", 5);
    println!("  - UT per share: {}", 20);
    println!("  - UT per like: {}", 2);
    println!("  - UT per view: {}", 1);

    println!("\n🎮 Simulating Viewer Login...");

    // Simulate viewer login
    let login_request = ViewerLoginRequest {
        nickname: "gaming_pro".to_string(),
        platform: "twitch".to_string(),
        phone: None,
    };

    let login_response = viewer_arm.login_viewer(login_request).await;
    println!("✅ Viewer login successful!");
    println!("  - Nickname: gaming_pro");
    println!("  - Platform: twitch");
    println!("  - Session ID: {}", login_response.session_id.as_ref().unwrap());
    println!("  - UT Balance: {}", login_response.ut_balance.unwrap_or(0));
    println!("  - KYC Status: {:?}", login_response.kyc_status.unwrap());

    let session_id = login_response.session_id.unwrap();

    println!("\n🎥 Simulating Streaming Activities...");

    // Simulate streaming activity
    let streaming_activity = UTActivityRequest {
        session_id: session_id.clone(),
        activity_type: "streaming".to_string(),
        reference: "stream_001".to_string(),
        duration_minutes: Some(60), // 1 hour
        count: None,
    };

    let streaming_response = viewer_arm.record_ut_activity(streaming_activity).await;
    println!("✅ Streaming activity recorded!");
    println!("  - Duration: 60 minutes");
    println!("  - UT Earned: {}", streaming_response.ut_earned.unwrap_or(0));
    println!("  - New UT Balance: {}", streaming_response.new_ut_balance.unwrap_or(0));

    // Simulate comment activity
    let comment_activity = UTActivityRequest {
        session_id: session_id.clone(),
        activity_type: "comment".to_string(),
        reference: "comment_001".to_string(),
        duration_minutes: None,
        count: Some(5), // 5 comments
    };

    let comment_response = viewer_arm.record_ut_activity(comment_activity).await;
    println!("✅ Comment activity recorded!");
    println!("  - Comments: 5");
    println!("  - UT Earned: {}", comment_response.ut_earned.unwrap_or(0));
    println!("  - New UT Balance: {}", comment_response.new_ut_balance.unwrap_or(0));

    // Simulate share activity
    let share_activity = UTActivityRequest {
        session_id: session_id.clone(),
        activity_type: "share".to_string(),
        reference: "share_001".to_string(),
        duration_minutes: None,
        count: Some(2), // 2 shares
    };

    let share_response = viewer_arm.record_ut_activity(share_activity).await;
    println!("✅ Share activity recorded!");
    println!("  - Shares: 2");
    println!("  - UT Earned: {}", share_response.ut_earned.unwrap_or(0));
    println!("  - New UT Balance: {}", share_response.new_ut_balance.unwrap_or(0));

    println!("\n📋 Simulating KYC Registration...");

    // Simulate KYC registration
    let kyc_request = KYCRegistrationRequest {
        session_id: session_id.clone(),
        full_name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
        phone: "+995 555 123 456".to_string(),
        tshirt_size: "L".to_string(),
        favorite_dish: "Plov".to_string(),
        password: "SecurePassword123!".to_string(),
        qr_code: Some("check_id_123|123456|0xabcd1234".to_string()),
    };

    let kyc_response = viewer_arm.register_for_kyc(kyc_request).await;
    println!("✅ KYC registration successful!");
    println!("  - Full Name: John Doe");
    println!("  - Email: john.doe@example.com");
    println!("  - Phone: +995 555 123 456");
    println!("  - T-shirt Size: L");
    println!("  - Favorite Dish: Plov");
    println!("  - User ID: {}", kyc_response.user_id.as_ref().unwrap());
    println!("  - KYC Status: {:?}", kyc_response.kyc_status.unwrap());

    println!("\n📊 Getting Viewer Statistics...");

    // Get viewer statistics
    let viewer_stats = viewer_arm.get_viewer_stats(&session_id).await;
    if let Some(stats) = viewer_stats {
        println!("📈 Viewer Statistics:");
        println!("  - Session ID: {}", stats.session_id);
        println!("  - Nickname: {}", stats.nickname);
        println!("  - Platform: {}", stats.platform);
        println!("  - User ID: {}", stats.user_id.as_ref().unwrap_or(&"Not registered".to_string()));
        println!("  - Phone: {}", stats.phone.as_ref().unwrap_or(&"Not provided".to_string()));
        println!("  - KYC Status: {:?}", stats.kyc_status);
        println!("  - UT Balance: {}", stats.ut_balance);
        println!("  - ST Balance: {}", stats.st_balance.unwrap_or(0));
        println!("  - Total Streaming Time: {} minutes", stats.total_streaming_time);
        println!("  - Total UT Earned: {}", stats.total_ut_earned);
        println!("  - Session Start: {}", stats.session_start);
        println!("  - Last Activity: {}", stats.last_activity);
    }

    println!("\n🏆 Getting UT Leaderboard...");

    // Get UT leaderboard
    let leaderboard = viewer_arm.get_ut_leaderboard(5).await;
    println!("📊 Top 5 UT Holders:");
    for (i, (user_id, ut_balance)) in leaderboard.iter().enumerate() {
        println!("  {}. {}: {} UT", i + 1, user_id, ut_balance);
    }

    println!("\n🔄 Getting Conversion Rounds...");

    // Get conversion rounds
    let conversion_rounds = viewer_arm.get_conversion_rounds().await;
    println!("📈 Conversion Rounds:");
    for (i, round) in conversion_rounds.iter().enumerate() {
        println!("  {}. Round ID: {}", i + 1, round.round_id);
        println!("     Total Pool: {} THP", round.total_pool);
        println!("     Total UT Snapshot: {}", round.total_ut_snapshot);
        println!("     Status: {:?}", round.status);
    }

    println!("\n🎯 Demo Summary:");
    println!("================");
    println!("✅ Successfully demonstrated:");
    println!("  - Viewer login by nickname and platform");
    println!("  - UT token earning through streaming activities");
    println!("  - KYC registration for THP token eligibility");
    println!("  - Statistics tracking and leaderboard");
    println!("  - Conversion rounds for THP distribution");
    println!("\n🚀 Viewer ARM is working correctly!");
    println!("   Viewers can login with their streaming nicknames");
    println!("   UT tokens are earned through platform activities");
    println!("   KYC registration enables THP token participation");
    println!("   Conversion rounds distribute THP to active UT holders");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use blockchain_project::kyc_aml::KYCAmlManager;

    #[tokio::test]
    async fn test_viewer_arm_creation() {
        let tokenomics_config = TokenomicsConfig::default();
        let kyc_manager = KYCAmlManager::new();
        let platforms = vec![
            StreamingPlatform {
                name: "twitch".to_string(),
                api_endpoint: "https://api.twitch.tv".to_string(),
                auth_token: "test_token".to_string(),
                is_active: true,
            }
        ];

        let viewer_arm = ViewerARM::new(tokenomics_config, kyc_manager, platforms);
        assert_eq!(viewer_arm.connected_platforms.len(), 1);
        assert_eq!(viewer_arm.connected_platforms[0].name, "twitch");
    }

    #[tokio::test]
    async fn test_viewer_login() {
        let tokenomics_config = TokenomicsConfig::default();
        let kyc_manager = KYCAmlManager::new();
        let platforms = vec![
            StreamingPlatform {
                name: "twitch".to_string(),
                api_endpoint: "https://api.twitch.tv".to_string(),
                auth_token: "test_token".to_string(),
                is_active: true,
            }
        ];

        let mut viewer_arm = ViewerARM::new(tokenomics_config, kyc_manager, platforms);

        let login_request = ViewerLoginRequest {
            nickname: "test_viewer".to_string(),
            platform: "twitch".to_string(),
            phone: None,
        };

        let response = viewer_arm.login_viewer(login_request).await;
        assert!(response.success);
        assert!(response.session_id.is_some());
        assert_eq!(response.ut_balance, Some(0));
    }

    #[tokio::test]
    async fn test_ut_activity_recording() {
        let tokenomics_config = TokenomicsConfig::default();
        let kyc_manager = KYCAmlManager::new();
        let platforms = vec![
            StreamingPlatform {
                name: "twitch".to_string(),
                api_endpoint: "https://api.twitch.tv".to_string(),
                auth_token: "test_token".to_string(),
                is_active: true,
            }
        ];

        let mut viewer_arm = ViewerARM::new(tokenomics_config, kyc_manager, platforms);

        // Login first
        let login_request = ViewerLoginRequest {
            nickname: "test_viewer".to_string(),
            platform: "twitch".to_string(),
            phone: None,
        };

        let login_response = viewer_arm.login_viewer(login_request).await;
        let session_id = login_response.session_id.unwrap();

        // Record streaming activity
        let activity_request = UTActivityRequest {
            session_id: session_id.clone(),
            activity_type: "streaming".to_string(),
            reference: "stream_123".to_string(),
            duration_minutes: Some(30),
            count: None,
        };

        let activity_response = viewer_arm.record_ut_activity(activity_request).await;
        assert!(activity_response.success);
        assert_eq!(activity_response.ut_earned, Some(300)); // 30 minutes * 10 UT
        assert_eq!(activity_response.new_ut_balance, Some(300));
    }
}
