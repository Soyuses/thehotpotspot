//! Stream Collector for The Hot Pot Spot
//! 
//! This module implements the stream collector service for tracking streaming activities
//! and awarding Utility Tokens (UT) based on user engagement.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use crate::new_tokenomics::{NewTokenomicsManager, UtEvent, UtEventType};
use crate::new_database::NewDatabaseManager;

/// Stream collector configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamCollectorConfig {
    /// UT tokens per minute of streaming
    pub ut_per_minute: u128,
    /// UT tokens per comment
    pub ut_per_comment: u128,
    /// UT tokens per share
    pub ut_per_share: u128,
    /// UT tokens per like
    pub ut_per_like: u128,
    /// UT tokens per view
    pub ut_per_view: u128,
    /// Maximum UT per day per user
    pub max_ut_per_day: u128,
    /// Minimum streaming duration in minutes to earn UT
    pub min_streaming_minutes: u32,
    /// Maximum streaming duration in minutes for UT calculation
    pub max_streaming_minutes: u32,
    /// Platforms to track
    pub supported_platforms: Vec<String>,
}

impl Default for StreamCollectorConfig {
    fn default() -> Self {
        Self {
            ut_per_minute: 10, // 10 UT per minute
            ut_per_comment: 5, // 5 UT per comment
            ut_per_share: 20,  // 20 UT per share
            ut_per_like: 2,    // 2 UT per like
            ut_per_view: 1,    // 1 UT per view
            max_ut_per_day: 1000, // Max 1000 UT per day
            min_streaming_minutes: 5, // Minimum 5 minutes
            max_streaming_minutes: 120, // Maximum 2 hours
            supported_platforms: vec!["twitch".to_string(), "youtube".to_string()],
        }
    }
}

/// Streaming session data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingSession {
    pub session_id: String,
    pub user_id: String,
    pub stream_id: String,
    pub platform: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_minutes: u32,
    pub ut_earned: u128,
    pub status: SessionStatus,
}

/// Session status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionStatus {
    Active,
    Completed,
    Abandoned,
}

/// Comment data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub comment_id: String,
    pub user_id: String,
    pub stream_id: String,
    pub platform: String,
    pub content: String,
    pub ut_earned: u128,
    pub timestamp: DateTime<Utc>,
}

/// Share data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Share {
    pub share_id: String,
    pub user_id: String,
    pub stream_id: String,
    pub platform: String,
    pub share_type: ShareType,
    pub ut_earned: u128,
    pub timestamp: DateTime<Utc>,
}

/// Share type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShareType {
    Social,
    Direct,
    Embed,
}

/// Like data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Like {
    pub like_id: String,
    pub user_id: String,
    pub stream_id: String,
    pub platform: String,
    pub ut_earned: u128,
    pub timestamp: DateTime<Utc>,
}

/// View data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct View {
    pub view_id: String,
    pub user_id: String,
    pub stream_id: String,
    pub platform: String,
    pub ut_earned: u128,
    pub timestamp: DateTime<Utc>,
}

/// Stream collector statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamCollectorStats {
    pub total_sessions: u64,
    pub active_sessions: u64,
    pub completed_sessions: u64,
    pub total_comments: u64,
    pub total_shares: u64,
    pub total_likes: u64,
    pub total_views: u64,
    pub total_ut_awarded: u128,
    pub last_updated: DateTime<Utc>,
}

/// Stream Collector Service
pub struct StreamCollector {
    config: StreamCollectorConfig,
    tokenomics_manager: Arc<RwLock<NewTokenomicsManager>>,
    database: Arc<NewDatabaseManager>,
    stats: Arc<RwLock<StreamCollectorStats>>,
    active_sessions: Arc<RwLock<HashMap<String, StreamingSession>>>,
}

impl StreamCollector {
    /// Create a new stream collector
    pub fn new(
        config: StreamCollectorConfig,
        tokenomics_manager: Arc<RwLock<NewTokenomicsManager>>,
        database: Arc<NewDatabaseManager>,
    ) -> Self {
        let stats = Arc::new(RwLock::new(StreamCollectorStats {
            total_sessions: 0,
            active_sessions: 0,
            completed_sessions: 0,
            total_comments: 0,
            total_shares: 0,
            total_likes: 0,
            total_views: 0,
            total_ut_awarded: 0,
            last_updated: Utc::now(),
        }));

        let active_sessions = Arc::new(RwLock::new(HashMap::new()));

        Self {
            config,
            tokenomics_manager,
            database,
            stats,
            active_sessions,
        }
    }

    /// Start a streaming session
    pub async fn start_streaming_session(
        &self,
        user_id: String,
        stream_id: String,
        platform: String,
    ) -> Result<StreamingSession, String> {
        // Validate platform
        if !self.config.supported_platforms.contains(&platform) {
            return Err(format!("Unsupported platform: {}", platform));
        }

        // Check if user already has an active session
        {
            let sessions = self.active_sessions.read().await;
            if sessions.values().any(|s| s.user_id == user_id && s.status == SessionStatus::Active) {
                return Err("User already has an active streaming session".to_string());
            }
        }

        let session_id = format!("session_{}_{}", user_id, Utc::now().timestamp());
        let start_time = Utc::now();

        let session = StreamingSession {
            session_id: session_id.clone(),
            user_id: user_id.clone(),
            stream_id: stream_id.clone(),
            platform: platform.clone(),
            start_time,
            end_time: None,
            duration_minutes: 0,
            ut_earned: 0,
            status: SessionStatus::Active,
        };

        // Add to active sessions
        {
            let mut sessions = self.active_sessions.write().await;
            sessions.insert(session_id.clone(), session.clone());
        }

        // Save to database
        self.database.create_streaming_session(
            session_id.clone(),
            user_id.parse::<i32>().map_err(|_| "Invalid user ID")?,
            stream_id.clone(),
            platform.clone(),
            start_time,
        ).await?;

        // Update statistics
        self.update_stats_new_session().await;

        Ok(session)
    }

    /// End a streaming session
    pub async fn end_streaming_session(
        &self,
        session_id: String,
    ) -> Result<StreamingSession, String> {
        let mut session = {
            let mut sessions = self.active_sessions.write().await;
            sessions.remove(&session_id)
                .ok_or("Session not found")?
        };

        let end_time = Utc::now();
        let duration_minutes = ((end_time - session.start_time).num_seconds() / 60) as u32;

        // Calculate UT earned
        let ut_earned = self.calculate_streaming_ut(duration_minutes);
        session.end_time = Some(end_time);
        session.duration_minutes = duration_minutes;
        session.ut_earned = ut_earned;
        session.status = SessionStatus::Completed;

        // Award UT tokens
        if ut_earned > 0 {
            self.award_ut_tokens(
                session.user_id.clone(),
                UtEventType::Streaming,
                ut_earned,
                session.stream_id.clone(),
                session.platform.clone(),
            ).await?;
        }

        // Update database
        self.database.update_streaming_session(
            session_id.clone(),
            end_time,
            duration_minutes,
            ut_earned as i64,
            "completed",
        ).await?;

        // Update statistics
        self.update_stats_completed_session(ut_earned).await;

        Ok(session)
    }

    /// Record a comment
    pub async fn record_comment(
        &self,
        user_id: String,
        stream_id: String,
        platform: String,
        content: String,
    ) -> Result<Comment, String> {
        // Validate platform
        if !self.config.supported_platforms.contains(&platform) {
            return Err(format!("Unsupported platform: {}", platform));
        }

        let comment_id = format!("comment_{}_{}", user_id, Utc::now().timestamp());
        let ut_earned = self.config.ut_per_comment;
        let timestamp = Utc::now();

        let comment = Comment {
            comment_id: comment_id.clone(),
            user_id: user_id.clone(),
            stream_id: stream_id.clone(),
            platform: platform.clone(),
            content: content.clone(),
            ut_earned,
            timestamp,
        };

        // Award UT tokens
        self.award_ut_tokens(
            user_id.clone(),
            UtEventType::Comment,
            ut_earned,
            stream_id.clone(),
            platform.clone(),
        ).await?;

        // Save to database
        self.database.create_comment(
            comment_id.clone(),
            user_id.parse::<i32>().map_err(|_| "Invalid user ID")?,
            stream_id.clone(),
            platform.clone(),
            content,
            ut_earned as i64,
        ).await?;

        // Update statistics
        self.update_stats_comment(ut_earned).await;

        Ok(comment)
    }

    /// Record a share
    pub async fn record_share(
        &self,
        user_id: String,
        stream_id: String,
        platform: String,
        share_type: ShareType,
    ) -> Result<Share, String> {
        // Validate platform
        if !self.config.supported_platforms.contains(&platform) {
            return Err(format!("Unsupported platform: {}", platform));
        }

        let share_id = format!("share_{}_{}", user_id, Utc::now().timestamp());
        let ut_earned = self.config.ut_per_share;
        let timestamp = Utc::now();

        let share = Share {
            share_id: share_id.clone(),
            user_id: user_id.clone(),
            stream_id: stream_id.clone(),
            platform: platform.clone(),
            share_type: share_type.clone(),
            ut_earned,
            timestamp,
        };

        // Award UT tokens
        self.award_ut_tokens(
            user_id.clone(),
            UtEventType::Share,
            ut_earned,
            stream_id.clone(),
            platform.clone(),
        ).await?;

        // Save to database
        self.database.create_share(
            share_id.clone(),
            user_id.parse::<i32>().map_err(|_| "Invalid user ID")?,
            stream_id.clone(),
            platform.clone(),
            match share_type {
                ShareType::Social => "social",
                ShareType::Direct => "direct",
                ShareType::Embed => "embed",
            }.to_string(),
            ut_earned as i64,
        ).await?;

        // Update statistics
        self.update_stats_share(ut_earned).await;

        Ok(share)
    }

    /// Record a like
    pub async fn record_like(
        &self,
        user_id: String,
        stream_id: String,
        platform: String,
    ) -> Result<Like, String> {
        // Validate platform
        if !self.config.supported_platforms.contains(&platform) {
            return Err(format!("Unsupported platform: {}", platform));
        }

        let like_id = format!("like_{}_{}", user_id, Utc::now().timestamp());
        let ut_earned = self.config.ut_per_like;
        let timestamp = Utc::now();

        let like = Like {
            like_id: like_id.clone(),
            user_id: user_id.clone(),
            stream_id: stream_id.clone(),
            platform: platform.clone(),
            ut_earned,
            timestamp,
        };

        // Award UT tokens
        self.award_ut_tokens(
            user_id.clone(),
            UtEventType::Like,
            ut_earned,
            stream_id.clone(),
            platform.clone(),
        ).await?;

        // Save to database
        self.database.create_like(
            like_id.clone(),
            user_id.parse::<i32>().map_err(|_| "Invalid user ID")?,
            stream_id.clone(),
            platform.clone(),
            ut_earned as i64,
        ).await?;

        // Update statistics
        self.update_stats_like(ut_earned).await;

        Ok(like)
    }

    /// Record a view
    pub async fn record_view(
        &self,
        user_id: String,
        stream_id: String,
        platform: String,
    ) -> Result<View, String> {
        // Validate platform
        if !self.config.supported_platforms.contains(&platform) {
            return Err(format!("Unsupported platform: {}", platform));
        }

        let view_id = format!("view_{}_{}", user_id, Utc::now().timestamp());
        let ut_earned = self.config.ut_per_view;
        let timestamp = Utc::now();

        let view = View {
            view_id: view_id.clone(),
            user_id: user_id.clone(),
            stream_id: stream_id.clone(),
            platform: platform.clone(),
            ut_earned,
            timestamp,
        };

        // Award UT tokens
        self.award_ut_tokens(
            user_id.clone(),
            UtEventType::View,
            ut_earned,
            stream_id.clone(),
            platform.clone(),
        ).await?;

        // Update statistics
        self.update_stats_view(ut_earned).await;

        Ok(view)
    }

    /// Get active sessions
    pub async fn get_active_sessions(&self) -> Vec<StreamingSession> {
        let sessions = self.active_sessions.read().await;
        sessions.values()
            .filter(|s| s.status == SessionStatus::Active)
            .cloned()
            .collect()
    }

    /// Get stream collector statistics
    pub async fn get_stats(&self) -> StreamCollectorStats {
        self.stats.read().await.clone()
    }

    /// Calculate UT for streaming duration
    fn calculate_streaming_ut(&self, duration_minutes: u32) -> u128 {
        if duration_minutes < self.config.min_streaming_minutes {
            return 0;
        }

        let capped_duration = duration_minutes.min(self.config.max_streaming_minutes);
        (capped_duration as u128) * self.config.ut_per_minute
    }

    /// Award UT tokens to user
    async fn award_ut_tokens(
        &self,
        user_id: String,
        event_type: UtEventType,
        units: u128,
        reference: String,
        platform: String,
    ) -> Result<(), String> {
        // Check daily limit
        if let Err(_) = self.check_daily_limit(&user_id, units).await {
            return Err("Daily UT limit exceeded".to_string());
        }

        let event_id = format!("ut_{}_{}", user_id, Utc::now().timestamp());

        // Create UT event in tokenomics manager
        {
            let mut manager = self.tokenomics_manager.write().await;
            manager.add_ut_event(UtEvent {
                event_id: event_id.clone(),
                user_id: user_id.clone(),
                event_type: event_type.clone(),
                units,
                reference: reference.clone(),
                timestamp: Utc::now(),
                platform: platform.clone(),
            })?;
        }

        // Save to database
        self.database.create_ut_event(
            event_id,
            user_id.parse::<i32>().map_err(|_| "Invalid user ID")?,
            match event_type {
                UtEventType::Streaming => "streaming",
                UtEventType::Comment => "comment",
                UtEventType::Share => "share",
                UtEventType::Like => "like",
                UtEventType::View => "view",
            },
            units as i64,
            reference,
            platform,
        ).await?;

        Ok(())
    }

    /// Check daily UT limit
    async fn check_daily_limit(&self, user_id: &str, _new_units: u128) -> Result<(), String> {
        // Get today's UT events for user
        let _today = Utc::now().date_naive();
        let _user_id_int = user_id.parse::<i32>().map_err(|_| "Invalid user ID")?;

        // This would need to be implemented in the database manager
        // For now, we'll assume the limit check passes
        Ok(())
    }

    /// Update statistics for new session
    async fn update_stats_new_session(&self) {
        let mut stats = self.stats.write().await;
        stats.total_sessions += 1;
        stats.active_sessions += 1;
        stats.last_updated = Utc::now();
    }

    /// Update statistics for completed session
    async fn update_stats_completed_session(&self, ut_earned: u128) {
        let mut stats = self.stats.write().await;
        stats.active_sessions = stats.active_sessions.saturating_sub(1);
        stats.completed_sessions += 1;
        stats.total_ut_awarded += ut_earned;
        stats.last_updated = Utc::now();
    }

    /// Update statistics for comment
    async fn update_stats_comment(&self, ut_earned: u128) {
        let mut stats = self.stats.write().await;
        stats.total_comments += 1;
        stats.total_ut_awarded += ut_earned;
        stats.last_updated = Utc::now();
    }

    /// Update statistics for share
    async fn update_stats_share(&self, ut_earned: u128) {
        let mut stats = self.stats.write().await;
        stats.total_shares += 1;
        stats.total_ut_awarded += ut_earned;
        stats.last_updated = Utc::now();
    }

    /// Update statistics for like
    async fn update_stats_like(&self, ut_earned: u128) {
        let mut stats = self.stats.write().await;
        stats.total_likes += 1;
        stats.total_ut_awarded += ut_earned;
        stats.last_updated = Utc::now();
    }

    /// Update statistics for view
    async fn update_stats_view(&self, ut_earned: u128) {
        let mut stats = self.stats.write().await;
        stats.total_views += 1;
        stats.total_ut_awarded += ut_earned;
        stats.last_updated = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_stream_collector_config_default() {
        let config = StreamCollectorConfig::default();
        assert_eq!(config.ut_per_minute, 10);
        assert_eq!(config.ut_per_comment, 5);
        assert_eq!(config.ut_per_share, 20);
        assert_eq!(config.ut_per_like, 2);
        assert_eq!(config.ut_per_view, 1);
        assert_eq!(config.max_ut_per_day, 1000);
        assert_eq!(config.min_streaming_minutes, 5);
        assert_eq!(config.max_streaming_minutes, 120);
        assert!(config.supported_platforms.contains(&"twitch".to_string()));
        assert!(config.supported_platforms.contains(&"youtube".to_string()));
    }

    #[tokio::test]
    async fn test_streaming_ut_calculation() {
        let config = StreamCollectorConfig::default();
        let collector = StreamCollector::new(
            config,
            Arc::new(RwLock::new(NewTokenomicsManager::new(TokenomicsConfig::default()))),
            Arc::new(NewDatabaseManager::new(NewDatabaseConfig::default()).await.unwrap()),
        );

        // Test minimum duration
        assert_eq!(collector.calculate_streaming_ut(3), 0); // Below minimum
        assert_eq!(collector.calculate_streaming_ut(5), 50); // Exactly minimum
        assert_eq!(collector.calculate_streaming_ut(10), 100); // Normal duration
        assert_eq!(collector.calculate_streaming_ut(120), 1200); // Maximum duration
        assert_eq!(collector.calculate_streaming_ut(150), 1200); // Above maximum (capped)
    }

    #[test]
    fn test_session_status() {
        let status = SessionStatus::Active;
        match status {
            SessionStatus::Active => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_share_type() {
        let share_type = ShareType::Social;
        match share_type {
            ShareType::Social => assert!(true),
            ShareType::Direct => assert!(false),
            ShareType::Embed => assert!(false),
        }
    }
}
