use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use std::sync::Arc;

use crate::new_tokenomics::{NewTokenomicsManager, UtEvent, UtEventType, ConversionRound};
use crate::tokenomics_config::TokenomicsConfig;
use crate::kyc_aml::{KYCAmlManager, KYCUser};
use crate::new_tokenomics::KycStatus;

impl From<crate::kyc_aml::KYCStatus> for KycStatus {
    fn from(status: crate::kyc_aml::KYCStatus) -> Self {
        match status {
            crate::kyc_aml::KYCStatus::NotStarted => KycStatus::NotRequired,
            crate::kyc_aml::KYCStatus::Pending => KycStatus::Pending,
            crate::kyc_aml::KYCStatus::Verified => KycStatus::Verified,
            crate::kyc_aml::KYCStatus::Rejected => KycStatus::Rejected,
            crate::kyc_aml::KYCStatus::Expired => KycStatus::Expired,
            crate::kyc_aml::KYCStatus::Suspended => KycStatus::Rejected, // Map suspended to rejected
        }
    }
}

/// Viewer ARM - Automated Workstation for viewers
#[derive(Clone)]
pub struct ViewerARM {
    /// Tokenomics manager
    pub tokenomics_manager: Arc<RwLock<NewTokenomicsManager>>,
    /// KYC/AML manager
    pub kyc_manager: Arc<RwLock<KYCAmlManager>>,
    /// Connected platforms
    pub connected_platforms: Vec<StreamingPlatform>,
    /// Viewer sessions
    pub viewer_sessions: HashMap<String, ViewerSession>,
}

/// Streaming platform integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingPlatform {
    /// Platform name (twitch, youtube, etc.)
    pub name: String,
    /// Platform API endpoint
    pub api_endpoint: String,
    /// Authentication token
    pub auth_token: String,
    /// Is active
    pub is_active: bool,
}

/// Viewer session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewerSession {
    /// Session ID
    pub session_id: String,
    /// Viewer nickname
    pub nickname: String,
    /// Platform
    pub platform: String,
    /// User ID (if registered)
    pub user_id: Option<String>,
    /// Phone number (if provided)
    pub phone: Option<String>,
    /// KYC status
    pub kyc_status: KycStatus,
    /// UT balance
    pub ut_balance: u128,
    /// ST balance (if KYC passed)
    pub st_balance: Option<u128>,
    /// Session start time
    pub session_start: DateTime<Utc>,
    /// Last activity
    pub last_activity: DateTime<Utc>,
    /// Total streaming time (minutes)
    pub total_streaming_time: u32,
    /// Total UT earned
    pub total_ut_earned: u128,
}

/// Viewer login request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewerLoginRequest {
    /// Viewer nickname
    pub nickname: String,
    /// Platform
    pub platform: String,
    /// Optional phone number for KYC
    pub phone: Option<String>,
}

/// Viewer login response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewerLoginResponse {
    /// Success status
    pub success: bool,
    /// Session ID
    pub session_id: Option<String>,
    /// Message
    pub message: String,
    /// UT balance
    pub ut_balance: Option<u128>,
    /// ST balance (if KYC passed)
    pub st_balance: Option<u128>,
    /// KYC status
    pub kyc_status: Option<KycStatus>,
}

/// UT activity request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTActivityRequest {
    /// Session ID
    pub session_id: String,
    /// Activity type
    pub activity_type: String,
    /// Reference (stream ID, comment ID, etc.)
    pub reference: String,
    /// Duration in minutes (for streaming)
    pub duration_minutes: Option<u32>,
    /// Count (for comments, likes, etc.)
    pub count: Option<u32>,
}

/// UT activity response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTActivityResponse {
    /// Success status
    pub success: bool,
    /// UT earned
    pub ut_earned: Option<u128>,
    /// New UT balance
    pub new_ut_balance: Option<u128>,
    /// Message
    pub message: String,
}

/// KYC registration request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KYCRegistrationRequest {
    /// Session ID
    pub session_id: String,
    /// Full name
    pub full_name: String,
    /// Email
    pub email: String,
    /// Phone number
    pub phone: String,
    /// T-shirt size
    pub tshirt_size: String,
    /// Favorite dish
    pub favorite_dish: String,
    /// Password
    pub password: String,
    /// QR code from check (for wallet linking)
    pub qr_code: Option<String>,
}

/// KYC registration response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KYCRegistrationResponse {
    /// Success status
    pub success: bool,
    /// User ID
    pub user_id: Option<String>,
    /// Message
    pub message: String,
    /// KYC status
    pub kyc_status: Option<KycStatus>,
}

impl ViewerARM {
    /// Create a new Viewer ARM
    pub fn new(
        tokenomics_config: TokenomicsConfig,
        kyc_manager: KYCAmlManager,
        connected_platforms: Vec<StreamingPlatform>,
    ) -> Self {
        let tokenomics_manager = Arc::new(RwLock::new(NewTokenomicsManager::new(tokenomics_config)));
        
        Self {
            tokenomics_manager,
            kyc_manager: Arc::new(RwLock::new(kyc_manager)),
            connected_platforms,
            viewer_sessions: HashMap::new(),
        }
    }

    /// Login viewer by nickname
    pub async fn login_viewer(&mut self, request: ViewerLoginRequest) -> ViewerLoginResponse {
        // Check if platform is connected
        if !self.connected_platforms.iter().any(|p| p.name == request.platform && p.is_active) {
            return ViewerLoginResponse {
                success: false,
                session_id: None,
                message: format!("Platform {} is not connected", request.platform),
                ut_balance: None,
                st_balance: None,
                kyc_status: None,
            };
        }

        // Create session
        let session_id = format!("SESSION_{}_{}", request.platform, Utc::now().timestamp());
        let now = Utc::now();

        // Check if user exists by phone (simplified for demo)
        let (user_id, kyc_status, st_balance) = if let Some(_phone) = &request.phone {
            // In real implementation, would search KYC manager for existing user
            (None, KycStatus::NotRequired, None)
        } else {
            (None, KycStatus::NotRequired, None)
        };

        // Get UT balance
        let ut_balance = if let Some(user_id) = &user_id {
            let tokenomics_manager = self.tokenomics_manager.read().await;
            tokenomics_manager.ut_holders.get(user_id)
                .map(|ut| ut.balance)
                .unwrap_or(0)
        } else {
            0
        };

        // Create session
        let session = ViewerSession {
            session_id: session_id.clone(),
            nickname: request.nickname.clone(),
            platform: request.platform.clone(),
            user_id,
            phone: request.phone.clone(),
            kyc_status: kyc_status.clone(),
            ut_balance,
            st_balance,
            session_start: now,
            last_activity: now,
            total_streaming_time: 0,
            total_ut_earned: 0,
        };

        self.viewer_sessions.insert(session_id.clone(), session);

        ViewerLoginResponse {
            success: true,
            session_id: Some(session_id),
            message: "Login successful".to_string(),
            ut_balance: Some(ut_balance),
            st_balance,
            kyc_status: Some(kyc_status),
        }
    }

    /// Record UT activity
    pub async fn record_ut_activity(&mut self, request: UTActivityRequest) -> UTActivityResponse {
        // Get session
        let session = match self.viewer_sessions.get_mut(&request.session_id) {
            Some(session) => session,
            None => {
                return UTActivityResponse {
                    success: false,
                    ut_earned: None,
                    new_ut_balance: None,
                    message: "Session not found".to_string(),
                };
            }
        };

        // Update last activity
        session.last_activity = Utc::now();

        // Determine activity type
        let event_type = match request.activity_type.as_str() {
            "streaming" => UtEventType::Streaming,
            "comment" => UtEventType::Comment,
            "share" => UtEventType::Share,
            "like" => UtEventType::Like,
            "view" => UtEventType::View,
            _ => {
                return UTActivityResponse {
                    success: false,
                    ut_earned: None,
                    new_ut_balance: None,
                    message: "Invalid activity type".to_string(),
                };
            }
        };

        // Calculate UT earned
        let ut_earned = match event_type {
            UtEventType::Streaming => {
                if let Some(duration) = request.duration_minutes {
                    session.total_streaming_time += duration;
                    if duration <= 45 {
                        1 * 100 // 1 SPOT per session (max 45 minutes)
                    } else {
                        0 // No UT for sessions longer than 45 minutes
                    }
                } else {
                    0
                }
            }
            UtEventType::Viewing => {
                if let Some(duration) = request.duration_minutes {
                    if duration >= 120 { // 2 hours
                        1 * 100 // 1 SPOT per 2 hours viewing
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            UtEventType::FifthVisit => {
                1 * 100 // 1 SPOT per 5th visit
            }
            UtEventType::Comment => {
                // For now, assume all comments are popular (50+ likes)
                // In real implementation, this would come from the streaming platform
                1 * 100 // 1 SPOT per popular comment (50+ likes)
            }
            UtEventType::Share => {
                1 * 100 // 1 SPOT per repost/share
            }
            UtEventType::Like => {
                0 // Legacy - not used in new tokenomics
            }
            UtEventType::View => {
                0 // Legacy - not used in new tokenomics
            }
        };

        // Update session
        session.ut_balance += ut_earned;
        session.total_ut_earned += ut_earned;

        // Record in tokenomics manager if user is registered
        if let Some(user_id) = &session.user_id {
            let ut_event = UtEvent {
                event_id: format!("EVENT_{}", Utc::now().timestamp()),
                user_id: user_id.clone(),
                event_type: event_type.clone(),
                units: ut_earned,
                timestamp: Utc::now(),
                reference: request.reference,
                platform: session.platform.clone(),
            };

            let mut tokenomics_manager = self.tokenomics_manager.write().await;
            if let Err(e) = tokenomics_manager.add_ut_event(ut_event) {
                return UTActivityResponse {
                    success: false,
                    ut_earned: None,
                    new_ut_balance: None,
                    message: format!("Failed to record UT event: {}", e),
                };
            }
        }

        UTActivityResponse {
            success: true,
            ut_earned: Some(ut_earned),
            new_ut_balance: Some(session.ut_balance),
            message: "UT activity recorded successfully".to_string(),
        }
    }

    /// Register viewer for KYC
    pub async fn register_for_kyc(&mut self, request: KYCRegistrationRequest) -> KYCRegistrationResponse {
        // Get session
        let session = match self.viewer_sessions.get_mut(&request.session_id) {
            Some(session) => session,
            None => {
                return KYCRegistrationResponse {
                    success: false,
                    user_id: None,
                    message: "Session not found".to_string(),
                    kyc_status: None,
                };
            }
        };

        // Create KYC user
        let user_id = format!("USER_{}", Utc::now().timestamp());
        let (first_name, last_name) = if let Some(space_pos) = request.full_name.find(' ') {
            (request.full_name[..space_pos].to_string(), request.full_name[space_pos+1..].to_string())
        } else {
            (request.full_name.clone(), "".to_string())
        };
        
        let _kyc_user = KYCUser {
            user_id: user_id.clone(),
            email: request.email.clone(),
            phone: Some(request.phone.clone()),
            first_name,
            last_name,
            date_of_birth: None,
            nationality: None,
            address: None,
            kyc_status: crate::kyc_aml::KYCStatus::Pending,
            kyc_level: crate::kyc_aml::KYCLevel::Basic,
            kyc_started_at: Some(Utc::now()),
            kyc_completed_at: None,
            kyc_expires_at: None,
            documents: Vec::new(),
            risk_score: 50,
            sanctions_check: false,
            pep_status: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login: None,
        };

        // Add to KYC manager (simplified for demo)
        // In real implementation, would use proper KYC manager methods
        let _kyc_manager = self.kyc_manager.write().await;

        // Update session
        session.user_id = Some(user_id.clone());
        session.phone = Some(request.phone);
        session.kyc_status = KycStatus::Pending;

        KYCRegistrationResponse {
            success: true,
            user_id: Some(user_id),
            message: "Registration successful. KYC verification pending.".to_string(),
            kyc_status: Some(KycStatus::Pending),
        }
    }

    /// Get viewer statistics
    pub async fn get_viewer_stats(&self, session_id: &str) -> Option<ViewerSession> {
        self.viewer_sessions.get(session_id).cloned()
    }

    /// Get conversion rounds
    pub async fn get_conversion_rounds(&self) -> Vec<ConversionRound> {
        let tokenomics_manager = self.tokenomics_manager.read().await;
        tokenomics_manager.conversion_rounds.clone()
    }

    /// Get UT leaderboard
    pub async fn get_ut_leaderboard(&self, limit: usize) -> Vec<(String, u128)> {
        let tokenomics_manager = self.tokenomics_manager.read().await;
        let mut leaderboard: Vec<(String, u128)> = tokenomics_manager
            .ut_holders
            .iter()
            .map(|(user_id, ut)| (user_id.clone(), ut.balance))
            .collect();
        
        leaderboard.sort_by(|a, b| b.1.cmp(&a.1));
        leaderboard.truncate(limit);
        leaderboard
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&mut self) {
        let now = Utc::now();
        let expired_sessions: Vec<String> = self.viewer_sessions
            .iter()
            .filter(|(_, session)| {
                now.signed_duration_since(session.last_activity).num_hours() > 24
            })
            .map(|(session_id, _)| session_id.clone())
            .collect();

        for session_id in expired_sessions {
            self.viewer_sessions.remove(&session_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kyc_aml::KYCAmlManager;

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
    async fn test_ut_activity() {
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
