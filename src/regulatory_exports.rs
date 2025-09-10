//! Regulatory Export Module for The Hot Pot Spot
//! 
//! Provides functionality to export regulatory reports in formats required by GSSS/CSD
//! for compliance with Georgian securities regulations.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::config;

/// Regulatory export formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    CSV,
    JSON,
    XML,
}

/// Token holder registry entry for regulatory export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolderRegistryEntry {
    pub holder_id: String,
    pub holder_name: String,
    pub holder_type: String, // "INDIVIDUAL", "CORPORATE", "CHARITY"
    pub address: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub security_tokens_held: u128, // в subunits
    pub utility_tokens_held: u128, // в subunits
    pub security_percentage: f64,
    pub utility_percentage: f64,
    pub registration_date: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub kyc_status: String, // "VERIFIED", "PENDING", "REJECTED"
    pub is_restricted: bool,
}

/// Token emission registry entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmissionRegistryEntry {
    pub emission_id: String,
    pub emission_date: DateTime<Utc>,
    pub emission_type: String, // "INITIAL", "SECONDARY", "BONUS"
    pub total_tokens_emitted: u128, // в subunits
    pub price_per_token_subunits: u128, // цена в subunits
    pub total_value_gel: f64, // общая стоимость в GEL
    pub currency: String,
    pub purpose: String,
    pub regulatory_approval: Option<String>,
    pub approval_date: Option<DateTime<Utc>>,
    pub status: String, // "APPROVED", "PENDING", "REJECTED"
}

/// Corporate action registry entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateActionEntry {
    pub action_id: String,
    pub action_type: String, // "DIVIDEND", "SPLIT", "MERGER", "ACQUISITION"
    pub action_date: DateTime<Utc>,
    pub record_date: DateTime<Utc>,
    pub ex_date: DateTime<Utc>,
    pub description: String,
    pub affected_holders: Vec<String>,
    pub token_ratio: Option<f64>, // для сплитов
    pub dividend_per_token_subunits: Option<u128>, // для дивидендов
    pub total_amount_subunits: u128,
    pub status: String, // "ANNOUNCED", "EXECUTED", "CANCELLED"
}

/// Regulatory export manager
pub struct RegulatoryExporter {
    pub holders: HashMap<String, HolderRegistryEntry>,
    pub emissions: Vec<EmissionRegistryEntry>,
    pub corporate_actions: Vec<CorporateActionEntry>,
}

impl RegulatoryExporter {
    pub fn new() -> Self {
        Self {
            holders: HashMap::new(),
            emissions: Vec::new(),
            corporate_actions: Vec::new(),
        }
    }

    /// Export holders registry to CSV format
    pub fn export_holders_csv(&self) -> String {
        let mut csv = String::new();
        
        // CSV Header
        csv.push_str("Holder ID,Holder Name,Holder Type,Address,Phone,Email,Security Tokens,Utility Tokens,Security %,Utility %,Registration Date,Last Updated,KYC Status,Restricted\n");
        
        for holder in self.holders.values() {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{},{:.2},{:.2},{},{},{},{}\n",
                holder.holder_id,
                holder.holder_name,
                holder.holder_type,
                holder.address,
                holder.phone.as_deref().unwrap_or(""),
                holder.email.as_deref().unwrap_or(""),
                holder.security_tokens_held,
                holder.utility_tokens_held,
                holder.security_percentage,
                holder.utility_percentage,
                holder.registration_date.format("%Y-%m-%d %H:%M:%S UTC"),
                holder.last_updated.format("%Y-%m-%d %H:%M:%S UTC"),
                holder.kyc_status,
                holder.is_restricted
            ));
        }
        
        csv
    }

    /// Export holders registry to JSON format
    pub fn export_holders_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.holders)
    }

    /// Export emissions registry to CSV format
    pub fn export_emissions_csv(&self) -> String {
        let mut csv = String::new();
        
        // CSV Header
        csv.push_str("Emission ID,Emission Date,Emission Type,Total Tokens,Price per Token,Total Value GEL,Currency,Purpose,Regulatory Approval,Approval Date,Status\n");
        
        for emission in &self.emissions {
            csv.push_str(&format!(
                "{},{},{},{},{},{:.2},{},{},{},{},{}\n",
                emission.emission_id,
                emission.emission_date.format("%Y-%m-%d %H:%M:%S UTC"),
                emission.emission_type,
                emission.total_tokens_emitted,
                emission.price_per_token_subunits,
                emission.total_value_gel,
                emission.currency,
                emission.purpose,
                emission.regulatory_approval.as_deref().unwrap_or(""),
                emission.approval_date.map(|d| d.format("%Y-%m-%d %H:%M:%S UTC").to_string()).unwrap_or_default(),
                emission.status
            ));
        }
        
        csv
    }

    /// Export emissions registry to JSON format
    pub fn export_emissions_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.emissions)
    }

    /// Export corporate actions registry to CSV format
    pub fn export_corporate_actions_csv(&self) -> String {
        let mut csv = String::new();
        
        // CSV Header
        csv.push_str("Action ID,Action Type,Action Date,Record Date,Ex Date,Description,Affected Holders,Token Ratio,Dividend per Token,Total Amount,Status\n");
        
        for action in &self.corporate_actions {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                action.action_id,
                action.action_type,
                action.action_date.format("%Y-%m-%d %H:%M:%S UTC"),
                action.record_date.format("%Y-%m-%d %H:%M:%S UTC"),
                action.ex_date.format("%Y-%m-%d %H:%M:%S UTC"),
                action.description,
                action.affected_holders.join(";"),
                action.token_ratio.map(|r| r.to_string()).unwrap_or_default(),
                action.dividend_per_token_subunits.map(|d| d.to_string()).unwrap_or_default(),
                action.total_amount_subunits,
                action.status
            ));
        }
        
        csv
    }

    /// Export corporate actions registry to JSON format
    pub fn export_corporate_actions_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.corporate_actions)
    }

    /// Generate comprehensive regulatory report
    pub fn generate_regulatory_report(&self, format: ExportFormat) -> Result<String, String> {
        match format {
            ExportFormat::CSV => {
                let mut report = String::new();
                report.push_str("=== THE HOT POT SPOT - REGULATORY REPORT ===\n");
                report.push_str(&format!("Generated: {}\n", Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
                report.push_str(&format!("Currency: {}\n", config::CURRENCY));
                report.push_str(&format!("Scale: {} subunits per token\n\n", config::SCALE));
                
                report.push_str("=== HOLDERS REGISTRY ===\n");
                report.push_str(&self.export_holders_csv());
                report.push_str("\n\n=== EMISSIONS REGISTRY ===\n");
                report.push_str(&self.export_emissions_csv());
                report.push_str("\n\n=== CORPORATE ACTIONS REGISTRY ===\n");
                report.push_str(&self.export_corporate_actions_csv());
                
                Ok(report)
            },
            ExportFormat::JSON => {
                let report = serde_json::json!({
                    "report_metadata": {
                        "generated_at": Utc::now(),
                        "currency": config::CURRENCY,
                        "scale": config::SCALE,
                        "regulator": config::regulatory::REGULATOR_NAME
                    },
                    "holders_registry": self.holders,
                    "emissions_registry": self.emissions,
                    "corporate_actions_registry": self.corporate_actions
                });
                
                serde_json::to_string_pretty(&report).map_err(|e| e.to_string())
            },
            ExportFormat::XML => {
                // XML export would be implemented here
                Err("XML export not yet implemented".to_string())
            }
        }
    }

    /// Add holder to registry
    pub fn add_holder(&mut self, holder: HolderRegistryEntry) {
        self.holders.insert(holder.holder_id.clone(), holder);
    }

    /// Add emission to registry
    pub fn add_emission(&mut self, emission: EmissionRegistryEntry) {
        self.emissions.push(emission);
    }

    /// Add corporate action to registry
    pub fn add_corporate_action(&mut self, action: CorporateActionEntry) {
        self.corporate_actions.push(action);
    }

    /// Update holder information
    pub fn update_holder(&mut self, holder_id: &str, updates: HolderUpdate) -> Result<(), String> {
        if let Some(holder) = self.holders.get_mut(holder_id) {
            if let Some(security_tokens) = updates.security_tokens {
                holder.security_tokens_held = security_tokens;
            }
            if let Some(utility_tokens) = updates.utility_tokens {
                holder.utility_tokens_held = utility_tokens;
            }
            if let Some(kyc_status) = updates.kyc_status {
                holder.kyc_status = kyc_status;
            }
            if let Some(is_restricted) = updates.is_restricted {
                holder.is_restricted = is_restricted;
            }
            holder.last_updated = Utc::now();
            Ok(())
        } else {
            Err(format!("Holder {} not found", holder_id))
        }
    }
}

/// Holder update structure
#[derive(Debug, Clone)]
pub struct HolderUpdate {
    pub security_tokens: Option<u128>,
    pub utility_tokens: Option<u128>,
    pub kyc_status: Option<String>,
    pub is_restricted: Option<bool>,
}

impl HolderUpdate {
    pub fn new() -> Self {
        Self {
            security_tokens: None,
            utility_tokens: None,
            kyc_status: None,
            is_restricted: None,
        }
    }

    pub fn with_security_tokens(mut self, tokens: u128) -> Self {
        self.security_tokens = Some(tokens);
        self
    }

    pub fn with_utility_tokens(mut self, tokens: u128) -> Self {
        self.utility_tokens = Some(tokens);
        self
    }

    pub fn with_kyc_status(mut self, status: String) -> Self {
        self.kyc_status = Some(status);
        self
    }

    pub fn with_restricted(mut self, restricted: bool) -> Self {
        self.is_restricted = Some(restricted);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holders_csv_export() {
        let mut exporter = RegulatoryExporter::new();
        
        let holder = HolderRegistryEntry {
            holder_id: "HOLDER_001".to_string(),
            holder_name: "Test Holder".to_string(),
            holder_type: "INDIVIDUAL".to_string(),
            address: "0x1234567890abcdef".to_string(),
            phone: Some("+995123456789".to_string()),
            email: Some("test@example.com".to_string()),
            security_tokens_held: 1000,
            utility_tokens_held: 1000,
            security_percentage: 10.0,
            utility_percentage: 10.0,
            registration_date: Utc::now(),
            last_updated: Utc::now(),
            kyc_status: "VERIFIED".to_string(),
            is_restricted: false,
        };
        
        exporter.add_holder(holder);
        let csv = exporter.export_holders_csv();
        
        assert!(csv.contains("HOLDER_001"));
        assert!(csv.contains("Test Holder"));
        assert!(csv.contains("1000"));
    }

    #[test]
    fn test_emissions_json_export() {
        let mut exporter = RegulatoryExporter::new();
        
        let emission = EmissionRegistryEntry {
            emission_id: "EMISSION_001".to_string(),
            emission_date: Utc::now(),
            emission_type: "INITIAL".to_string(),
            total_tokens_emitted: 10000,
            price_per_token_subunits: 500, // 5.00 GEL
            total_value_gel: 50000.0,
            currency: "GEL".to_string(),
            purpose: "Initial token offering".to_string(),
            regulatory_approval: Some("GSSS_APPROVAL_001".to_string()),
            approval_date: Some(Utc::now()),
            status: "APPROVED".to_string(),
        };
        
        exporter.add_emission(emission);
        let json = exporter.export_emissions_json().unwrap();
        
        assert!(json.contains("EMISSION_001"));
        assert!(json.contains("10000"));
        assert!(json.contains("500"));
    }
}

