//! New Database Module for The Hot Pot Spot
//! 
//! This module provides database operations for the new ST/UT tokenomics model.

use serde::{Serialize, Deserialize};
// use std::collections::HashMap;
use chrono::{DateTime, Utc};
use std::time::SystemTime;
use tokio_postgres::{Client, NoTls, Row};
use crate::new_tokenomics::{
    SaleRecord, SaleStatus, UtEvent, UtEventType,
    StMinting, ConversionRound, ConversionRoundStatus, ConversionAllocation,
    UserBalance, KycStatus
};

/// Database configuration for new tokenomics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewDatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub max_connections: u32,
}

impl Default for NewDatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            database: "thehotpotspot".to_string(),
            username: "postgres".to_string(),
            password: "password".to_string(),
            max_connections: 10,
        }
    }
}

/// User record from database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub phone_hash: String,
    pub wallet_addr: Option<String>,
    pub kyc_status: String,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub t_shirt_size: Option<String>,
    pub favorite_dish: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// New Database Manager
pub struct NewDatabaseManager {
    client: Client,
    config: NewDatabaseConfig,
}

impl NewDatabaseManager {
    /// Create a new database manager
    pub async fn new(config: NewDatabaseConfig) -> Result<Self, String> {
        let connection_string = format!(
            "host={} port={} dbname={} user={} password={}",
            config.host, config.port, config.database, config.username, config.password
        );

        let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        // Spawn connection task
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Database connection error: {}", e);
            }
        });

        Ok(Self { client, config })
    }

    /// Create user
    pub async fn create_user(
        &self,
        phone_hash: String,
        wallet_addr: Option<String>,
        full_name: Option<String>,
        email: Option<String>,
        t_shirt_size: Option<String>,
        favorite_dish: Option<String>,
    ) -> Result<User, String> {
        let query = r#"
            INSERT INTO users (phone_hash, wallet_addr, full_name, email, t_shirt_size, favorite_dish)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, phone_hash, wallet_addr, kyc_status, full_name, email, t_shirt_size, favorite_dish, created_at, updated_at
        "#;

        let row = self.client
            .query_one(query, &[&phone_hash, &wallet_addr, &full_name, &email, &t_shirt_size, &favorite_dish])
            .await
            .map_err(|e| format!("Failed to create user: {}", e))?;

        Ok(Self::row_to_user(&row))
    }

    /// Get user by phone hash
    pub async fn get_user_by_phone(&self, phone_hash: &str) -> Result<Option<User>, String> {
        let query = "SELECT * FROM users WHERE phone_hash = $1";
        let rows = self.client
            .query(query, &[&phone_hash])
            .await
            .map_err(|e| format!("Failed to get user: {}", e))?;

        Ok(rows.first().map(Self::row_to_user))
    }

    /// Get user by wallet address
    pub async fn get_user_by_wallet(&self, wallet_addr: &str) -> Result<Option<User>, String> {
        let query = "SELECT * FROM users WHERE wallet_addr = $1";
        let rows = self.client
            .query(query, &[&wallet_addr])
            .await
            .map_err(|e| format!("Failed to get user: {}", e))?;

        Ok(rows.first().map(Self::row_to_user))
    }

    /// Update user KYC status
    pub async fn update_user_kyc_status(&self, user_id: i32, kyc_status: &str) -> Result<(), String> {
        let query = "UPDATE users SET kyc_status = $1, updated_at = NOW() WHERE id = $2";
        self.client
            .execute(query, &[&kyc_status, &user_id])
            .await
            .map_err(|e| format!("Failed to update KYC status: {}", e))?;

        Ok(())
    }

    /// Create sale record
    pub async fn create_sale(
        &self,
        sale_id: String,
        node_id: String,
        user_id: Option<i32>,
        amount_gel: f64,
        st_units: i64,
        check_address: String,
        activation_code_hash: String,
    ) -> Result<SaleRecord, String> {
        let query = r#"
            INSERT INTO sales (sale_id, node_id, user_id, amount_gel, st_units, check_address, activation_code_hash)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING sale_id, node_id, user_id, amount_gel, st_units, check_address, activation_code_hash, status, created_at, updated_at
        "#;

        let row = self.client
            .query_one(query, &[&sale_id, &node_id, &user_id, &amount_gel, &st_units, &check_address, &activation_code_hash])
            .await
            .map_err(|e| format!("Failed to create sale: {}", e))?;

        Ok(Self::row_to_sale_record(&row))
    }

    /// Get sale by check address
    pub async fn get_sale_by_check_address(&self, check_address: &str) -> Result<Option<SaleRecord>, String> {
        let query = "SELECT * FROM sales WHERE check_address = $1";
        let rows = self.client
            .query(query, &[&check_address])
            .await
            .map_err(|e| format!("Failed to get sale: {}", e))?;

        Ok(rows.first().map(Self::row_to_sale_record))
    }

    /// Update sale status
    pub async fn update_sale_status(&self, sale_id: &str, status: &str) -> Result<(), String> {
        let query = "UPDATE sales SET status = $1, updated_at = NOW() WHERE sale_id = $2";
        self.client
            .execute(query, &[&status, &sale_id])
            .await
            .map_err(|e| format!("Failed to update sale status: {}", e))?;

        Ok(())
    }

    /// Create ST minting record
    pub async fn create_st_minting(
        &self,
        mint_id: String,
        sale_id: String,
        units: i64,
        to_address: String,
        transaction_hash: String,
    ) -> Result<StMinting, String> {
        let query = r#"
            INSERT INTO st_mintings (mint_id, sale_id, units, to_address, transaction_hash)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING mint_id, sale_id, units, to_address, transaction_hash, created_at
        "#;

        let row = self.client
            .query_one(query, &[&mint_id, &sale_id, &units, &to_address, &transaction_hash])
            .await
            .map_err(|e| format!("Failed to create ST minting: {}", e))?;

        Ok(Self::row_to_st_minting(&row))
    }

    /// Create UT event
    pub async fn create_ut_event(
        &self,
        event_id: String,
        user_id: i32,
        event_type: &str,
        units: i64,
        reference: String,
        platform: String,
    ) -> Result<UtEvent, String> {
        let query = r#"
            INSERT INTO ut_events (event_id, user_id, event_type, units, reference, platform)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING event_id, user_id, event_type, units, reference, platform, created_at
        "#;

        let row = self.client
            .query_one(query, &[&event_id, &user_id, &event_type, &units, &reference, &platform])
            .await
            .map_err(|e| format!("Failed to create UT event: {}", e))?;

        Ok(Self::row_to_ut_event(&row))
    }

    /// Get user balance summary
    pub async fn get_user_balance(&self, user_id: i32) -> Result<UserBalance, String> {
        let query = "SELECT * FROM user_balance_summary WHERE user_id = $1";
        let rows = self.client
            .query(query, &[&user_id])
            .await
            .map_err(|e| format!("Failed to get user balance: {}", e))?;

        let row = rows.first().ok_or("User not found")?;
        Ok(Self::row_to_user_balance(&row))
    }

    /// Create conversion round
    pub async fn create_conversion_round(
        &self,
        round_id: String,
        total_pool: i64,
        total_ut_snapshot: i64,
    ) -> Result<ConversionRound, String> {
        let query = r#"
            INSERT INTO conversion_rounds (round_id, total_pool, total_ut_snapshot)
            VALUES ($1, $2, $3)
            RETURNING round_id, total_pool, total_ut_snapshot, distributed, status, created_at, completed_at
        "#;

        let row = self.client
            .query_one(query, &[&round_id, &total_pool, &total_ut_snapshot])
            .await
            .map_err(|e| format!("Failed to create conversion round: {}", e))?;

        Ok(Self::row_to_conversion_round(&row))
    }

    /// Create conversion allocation
    pub async fn create_conversion_allocation(
        &self,
        round_id: String,
        user_id: i32,
        allocated_units: i64,
        kyc_status: &str,
        transaction_hash: Option<String>,
    ) -> Result<ConversionAllocation, String> {
        let query = r#"
            INSERT INTO conversion_allocations (round_id, user_id, allocated_units, kyc_status, transaction_hash)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, round_id, user_id, allocated_units, kyc_status, transaction_hash, created_at
        "#;

        let row = self.client
            .query_one(query, &[&round_id, &user_id, &allocated_units, &kyc_status, &transaction_hash])
            .await
            .map_err(|e| format!("Failed to create conversion allocation: {}", e))?;

        Ok(Self::row_to_conversion_allocation(&row))
    }

    /// Get reserved ST amount
    pub async fn get_reserved_st(&self) -> Result<i64, String> {
        let query = "SELECT total_reserved FROM reserved_st ORDER BY last_updated DESC LIMIT 1";
        let rows = self.client
            .query(query, &[])
            .await
            .map_err(|e| format!("Failed to get reserved ST: {}", e))?;

        let row = rows.first().ok_or("No reserved ST record found")?;
        Ok(row.get::<_, i64>("total_reserved"))
    }

    /// Get total UT snapshot for conversion
    pub async fn get_total_ut_snapshot(&self, min_ut: i64) -> Result<i64, String> {
        let query = "SELECT SUM(units) as total FROM ut_balances WHERE units >= $1";
        let rows = self.client
            .query(query, &[&min_ut])
            .await
            .map_err(|e| format!("Failed to get UT snapshot: {}", e))?;

        let row = rows.first().ok_or("No UT balances found")?;
        Ok(row.get::<_, Option<i64>>("total").unwrap_or(0))
    }

    /// Get UT holders for conversion
    pub async fn get_ut_holders_for_conversion(&self, min_ut: i64) -> Result<Vec<(i32, i64)>, String> {
        let query = "SELECT user_id, units FROM ut_balances WHERE units >= $1 ORDER BY units DESC";
        let rows = self.client
            .query(query, &[&min_ut])
            .await
            .map_err(|e| format!("Failed to get UT holders: {}", e))?;

        let mut holders = Vec::new();
        for row in rows {
            let user_id: i32 = row.get("user_id");
            let units: i64 = row.get("units");
            holders.push((user_id, units));
        }

        Ok(holders)
    }

    /// Update conversion round status
    pub async fn update_conversion_round_status(&self, round_id: &str, status: &str) -> Result<(), String> {
        let query = "UPDATE conversion_rounds SET status = $1, completed_at = NOW() WHERE round_id = $2";
        self.client
            .execute(query, &[&status, &round_id])
            .await
            .map_err(|e| format!("Failed to update conversion round status: {}", e))?;

        Ok(())
    }

    /// Get conversion round statistics
    pub async fn get_conversion_stats(&self) -> Result<Vec<ConversionRoundStats>, String> {
        let query = "SELECT * FROM conversion_round_stats ORDER BY created_at DESC";
        let rows = self.client
            .query(query, &[])
            .await
            .map_err(|e| format!("Failed to get conversion stats: {}", e))?;

        let mut stats = Vec::new();
        for row in rows {
            stats.push(Self::row_to_conversion_round_stats(&row));
        }

        Ok(stats)
    }

    /// Create streaming session
    pub async fn create_streaming_session(
        &self,
        session_id: String,
        user_id: i32,
        stream_id: String,
        platform: String,
        start_time: DateTime<Utc>,
    ) -> Result<(), String> {
        let query = r#"
            INSERT INTO streaming_sessions (session_id, user_id, stream_id, platform, start_time)
            VALUES ($1, $2, $3, $4, $5)
        "#;

        let start_time_system = SystemTime::from(start_time);
        self.client
            .execute(query, &[&session_id, &user_id, &stream_id, &platform, &start_time_system])
            .await
            .map_err(|e| format!("Failed to create streaming session: {}", e))?;

        Ok(())
    }

    /// Update streaming session
    pub async fn update_streaming_session(
        &self,
        session_id: String,
        end_time: DateTime<Utc>,
        duration_minutes: u32,
        ut_earned: i64,
        status: &str,
    ) -> Result<(), String> {
        let query = r#"
            UPDATE streaming_sessions 
            SET end_time = $1, duration_minutes = $2, ut_earned = $3, status = $4
            WHERE session_id = $5
        "#;

        let end_time_system = SystemTime::from(end_time);
        self.client
            .execute(query, &[&end_time_system, &(duration_minutes as i32), &ut_earned, &status, &session_id])
            .await
            .map_err(|e| format!("Failed to update streaming session: {}", e))?;

        Ok(())
    }

    /// Create comment
    pub async fn create_comment(
        &self,
        comment_id: String,
        user_id: i32,
        stream_id: String,
        platform: String,
        content: String,
        ut_earned: i64,
    ) -> Result<(), String> {
        let query = r#"
            INSERT INTO comments (comment_id, user_id, stream_id, platform, content, ut_earned)
            VALUES ($1, $2, $3, $4, $5, $6)
        "#;

        self.client
            .execute(query, &[&comment_id, &user_id, &stream_id, &platform, &content, &ut_earned])
            .await
            .map_err(|e| format!("Failed to create comment: {}", e))?;

        Ok(())
    }

    /// Create share
    pub async fn create_share(
        &self,
        share_id: String,
        user_id: i32,
        stream_id: String,
        platform: String,
        share_type: String,
        ut_earned: i64,
    ) -> Result<(), String> {
        let query = r#"
            INSERT INTO shares (share_id, user_id, stream_id, platform, share_type, ut_earned)
            VALUES ($1, $2, $3, $4, $5, $6)
        "#;

        self.client
            .execute(query, &[&share_id, &user_id, &stream_id, &platform, &share_type, &ut_earned])
            .await
            .map_err(|e| format!("Failed to create share: {}", e))?;

        Ok(())
    }

    /// Create like
    pub async fn create_like(
        &self,
        like_id: String,
        user_id: i32,
        stream_id: String,
        platform: String,
        ut_earned: i64,
    ) -> Result<(), String> {
        let query = r#"
            INSERT INTO likes (like_id, user_id, stream_id, platform, ut_earned)
            VALUES ($1, $2, $3, $4, $5)
        "#;

        self.client
            .execute(query, &[&like_id, &user_id, &stream_id, &platform, &ut_earned])
            .await
            .map_err(|e| format!("Failed to create like: {}", e))?;

        Ok(())
    }

    // Helper methods to convert database rows to structs

    fn row_to_user(row: &Row) -> User {
        User {
            id: row.get("id"),
            phone_hash: row.get("phone_hash"),
            wallet_addr: row.get("wallet_addr"),
            kyc_status: row.get("kyc_status"),
            full_name: row.get("full_name"),
            email: row.get("email"),
            t_shirt_size: row.get("t_shirt_size"),
            favorite_dish: row.get("favorite_dish"),
            created_at: DateTime::from(SystemTime::from(row.get::<_, SystemTime>("created_at"))),
            updated_at: DateTime::from(SystemTime::from(row.get::<_, SystemTime>("updated_at"))),
        }
    }

    fn row_to_sale_record(row: &Row) -> SaleRecord {
        SaleRecord {
            sale_id: row.get("sale_id"),
            node_id: row.get("node_id"),
            user_id: row.get("user_id"),
            amount_gel: row.get::<_, f64>("amount_gel"),
            st_units: row.get::<_, i64>("st_units") as u128,
            check_address: row.get("check_address"),
            activation_code_hash: row.get("activation_code_hash"),
            timestamp: DateTime::from(SystemTime::from(row.get::<_, SystemTime>("created_at"))),
            status: match row.get::<_, String>("status").as_str() {
                "pending" => SaleStatus::Pending,
                "processed" => SaleStatus::Processed,
                "claimed" => SaleStatus::Claimed,
                "expired" => SaleStatus::Expired,
                _ => SaleStatus::Pending,
            },
        }
    }

    fn row_to_st_minting(row: &Row) -> StMinting {
        StMinting {
            mint_id: row.get("mint_id"),
            sale_id: row.get("sale_id"),
            units: row.get::<_, i64>("units") as u128,
            to_address: row.get("to_address"),
            transaction_hash: row.get("transaction_hash"),
            timestamp: DateTime::from(SystemTime::from(row.get::<_, SystemTime>("created_at"))),
        }
    }

    fn row_to_ut_event(row: &Row) -> UtEvent {
        UtEvent {
            event_id: row.get("event_id"),
            user_id: row.get::<_, i32>("user_id").to_string(),
            event_type: match row.get::<_, String>("event_type").as_str() {
                "streaming" => UtEventType::Streaming,
                "comment" => UtEventType::Comment,
                "share" => UtEventType::Share,
                "like" => UtEventType::Like,
                "view" => UtEventType::View,
                _ => UtEventType::View,
            },
            units: row.get::<_, i64>("units") as u128,
            reference: row.get("reference"),
            timestamp: DateTime::from(SystemTime::from(row.get::<_, SystemTime>("created_at"))),
            platform: row.get("platform"),
        }
    }

    fn row_to_user_balance(row: &Row) -> UserBalance {
        UserBalance {
            user_id: row.get::<_, i32>("user_id").to_string(),
            st_balance: row.get::<_, i64>("st_balance") as u128,
            ut_balance: row.get::<_, i64>("ut_balance") as u128,
            claimable_st: row.get::<_, i64>("claimable_st") as u128,
            voting_power: row.get::<_, i64>("voting_power") as u128,
            kyc_status: match row.get::<_, String>("kyc_status").as_str() {
                "not_required" => KycStatus::NotRequired,
                "pending" => KycStatus::Pending,
                "verified" => KycStatus::Verified,
                "rejected" => KycStatus::Rejected,
                "expired" => KycStatus::Expired,
                _ => KycStatus::Pending,
            },
            last_updated: DateTime::from(SystemTime::from(row.get::<_, SystemTime>("updated_at"))),
        }
    }

    fn row_to_conversion_round(row: &Row) -> ConversionRound {
        ConversionRound {
            round_id: row.get("round_id"),
            total_pool: row.get::<_, i64>("total_pool") as u128,
            total_ut_snapshot: row.get::<_, i64>("total_ut_snapshot") as u128,
            distributed: row.get::<_, i64>("distributed") as u128,
            timestamp: DateTime::from(SystemTime::from(row.get::<_, SystemTime>("created_at"))),
            status: match row.get::<_, String>("status").as_str() {
                "pending" => ConversionRoundStatus::Pending,
                "in_progress" => ConversionRoundStatus::InProgress,
                "completed" => ConversionRoundStatus::Completed,
                "failed" => ConversionRoundStatus::Failed,
                _ => ConversionRoundStatus::Pending,
            },
        }
    }

    fn row_to_conversion_allocation(row: &Row) -> ConversionAllocation {
        ConversionAllocation {
            round_id: row.get("round_id"),
            user_id: row.get::<_, i32>("user_id").to_string(),
            allocated_units: row.get::<_, i64>("allocated_units") as u128,
            kyc_status: match row.get::<_, String>("kyc_status").as_str() {
                "not_required" => KycStatus::NotRequired,
                "pending" => KycStatus::Pending,
                "verified" => KycStatus::Verified,
                "rejected" => KycStatus::Rejected,
                "expired" => KycStatus::Expired,
                _ => KycStatus::Pending,
            },
            transaction_hash: row.get("transaction_hash"),
            timestamp: DateTime::from(SystemTime::from(row.get::<_, SystemTime>("created_at"))),
        }
    }

    fn row_to_conversion_round_stats(row: &Row) -> ConversionRoundStats {
        ConversionRoundStats {
            round_id: row.get("round_id"),
            total_pool: row.get::<_, i64>("total_pool") as u128,
            total_ut_snapshot: row.get::<_, i64>("total_ut_snapshot") as u128,
            distributed: row.get::<_, i64>("distributed") as u128,
            status: row.get("status"),
            created_at: DateTime::from(SystemTime::from(row.get::<_, SystemTime>("created_at"))),
            completed_at: row.get::<_, Option<SystemTime>>("completed_at").map(|st| DateTime::from(SystemTime::from(st))),
            total_allocations: row.get::<_, i64>("total_allocations") as usize,
            verified_allocations: row.get::<_, i64>("verified_allocations") as usize,
            completed_allocations: row.get::<_, i64>("completed_allocations") as usize,
        }
    }
}

/// Conversion round statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionRoundStats {
    pub round_id: String,
    pub total_pool: u128,
    pub total_ut_snapshot: u128,
    pub distributed: u128,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub total_allocations: usize,
    pub verified_allocations: usize,
    pub completed_allocations: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_config() {
        let config = NewDatabaseConfig::default();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 5432);
        assert_eq!(config.database, "thehotpotspot");
    }

    #[tokio::test]
    async fn test_connection_string() {
        let config = NewDatabaseConfig::default();
        let connection_string = format!(
            "host={} port={} dbname={} user={} password={}",
            config.host, config.port, config.database, config.username, config.password
        );
        assert!(connection_string.contains("localhost"));
        assert!(connection_string.contains("5432"));
        assert!(connection_string.contains("thehotpotspot"));
    }
}
