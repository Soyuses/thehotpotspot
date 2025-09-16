use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use std::sync::Arc;

use crate::new_tokenomics::NewTokenomicsManager;
use crate::tokenomics_config::TokenomicsConfig;

/// DAO Governance System for The Hot Pot Spot
#[derive(Debug, Clone)]
pub struct GovernanceDAO {
    /// Tokenomics manager
    pub tokenomics_manager: Arc<RwLock<NewTokenomicsManager>>,
    /// Configuration
    pub config: TokenomicsConfig,
    /// Active proposals
    pub proposals: HashMap<String, Proposal>,
    /// Votes
    pub votes: HashMap<String, Vec<Vote>>,
    /// Governance parameters
    pub parameters: GovernanceParameters,
}

/// Governance proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    /// Proposal ID
    pub proposal_id: String,
    /// Title
    pub title: String,
    /// Description
    pub description: String,
    /// Proposal type
    pub proposal_type: ProposalType,
    /// Creator
    pub creator: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Voting start time
    pub voting_start: DateTime<Utc>,
    /// Voting end time
    pub voting_end: DateTime<Utc>,
    /// Status
    pub status: ProposalStatus,
    /// Execution data (for executable proposals)
    pub execution_data: Option<ExecutionData>,
    /// Results
    pub results: Option<ProposalResults>,
}

/// Proposal type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProposalType {
    /// Tokenomics parameter change
    TokenomicsChange {
        parameter: String,
        old_value: String,
        new_value: String,
    },
    /// Treasury allocation
    TreasuryAllocation {
        recipient: String,
        amount: u128,
        purpose: String,
    },
    /// Protocol upgrade
    ProtocolUpgrade {
        version: String,
        description: String,
    },
    /// Emergency action
    EmergencyAction {
        action: String,
        reason: String,
    },
    /// General proposal
    General {
        category: String,
    },
}

/// Proposal status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProposalStatus {
    /// Draft
    Draft,
    /// Active voting
    Active,
    /// Passed
    Passed,
    /// Rejected
    Rejected,
    /// Executed
    Executed,
    /// Cancelled
    Cancelled,
}

/// Execution data for proposals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionData {
    /// Execution timestamp
    pub executed_at: Option<DateTime<Utc>>,
    /// Executor
    pub executor: Option<String>,
    /// Transaction hash
    pub transaction_hash: Option<String>,
    /// Execution result
    pub result: Option<String>,
}

/// Proposal results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalResults {
    /// Total votes
    pub total_votes: u32,
    /// Yes votes
    pub yes_votes: u32,
    /// No votes
    pub no_votes: u32,
    /// Abstain votes
    pub abstain_votes: u32,
    /// Total voting power
    pub total_voting_power: u128,
    /// Yes voting power
    pub yes_voting_power: u128,
    /// No voting power
    pub no_voting_power: u128,
    /// Abstain voting power
    pub abstain_voting_power: u128,
    /// Quorum reached
    pub quorum_reached: bool,
    /// Majority achieved
    pub majority_achieved: bool,
}

/// Vote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    /// Vote ID
    pub vote_id: String,
    /// Proposal ID
    pub proposal_id: String,
    /// Voter
    pub voter: String,
    /// Vote choice
    pub choice: VoteChoice,
    /// Voting power
    pub voting_power: u128,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Transaction hash
    pub transaction_hash: Option<String>,
}

/// Vote choice
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VoteChoice {
    Yes,
    No,
    Abstain,
}

/// Governance parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceParameters {
    /// Minimum voting period (hours)
    pub min_voting_period_hours: u32,
    /// Maximum voting period (hours)
    pub max_voting_period_hours: u32,
    /// Quorum threshold (percentage)
    pub quorum_threshold_percent: u8,
    /// Majority threshold (percentage)
    pub majority_threshold_percent: u8,
    /// Minimum UT balance to create proposal
    pub min_ut_to_create_proposal: u128,
    /// Minimum UT balance to vote
    pub min_ut_to_vote: u128,
    /// Proposal deposit (in UT)
    pub proposal_deposit: u128,
    /// Execution delay (hours)
    pub execution_delay_hours: u32,
}

impl Default for GovernanceParameters {
    fn default() -> Self {
        Self {
            min_voting_period_hours: 24,
            max_voting_period_hours: 168, // 7 days
            quorum_threshold_percent: 20,
            majority_threshold_percent: 51,
            min_ut_to_create_proposal: 1000,
            min_ut_to_vote: 100,
            proposal_deposit: 100,
            execution_delay_hours: 24,
        }
    }
}

/// Create proposal request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProposalRequest {
    /// Title
    pub title: String,
    /// Description
    pub description: String,
    /// Proposal type
    pub proposal_type: ProposalType,
    /// Voting duration (hours)
    pub voting_duration_hours: u32,
    /// Creator
    pub creator: String,
}

/// Create proposal response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProposalResponse {
    /// Success status
    pub success: bool,
    /// Proposal ID
    pub proposal_id: Option<String>,
    /// Message
    pub message: String,
}

/// Vote request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteRequest {
    /// Proposal ID
    pub proposal_id: String,
    /// Voter
    pub voter: String,
    /// Vote choice
    pub choice: VoteChoice,
}

/// Vote response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteResponse {
    /// Success status
    pub success: bool,
    /// Vote ID
    pub vote_id: Option<String>,
    /// Voting power
    pub voting_power: Option<u128>,
    /// Message
    pub message: String,
}

impl GovernanceDAO {
    /// Create a new Governance DAO
    pub fn new(
        tokenomics_manager: Arc<RwLock<NewTokenomicsManager>>,
        config: TokenomicsConfig,
    ) -> Self {
        Self {
            tokenomics_manager,
            config,
            proposals: HashMap::new(),
            votes: HashMap::new(),
            parameters: GovernanceParameters::default(),
        }
    }

    /// Create a new proposal
    pub async fn create_proposal(&mut self, request: CreateProposalRequest) -> CreateProposalResponse {
        // Validate voting duration
        if request.voting_duration_hours < self.parameters.min_voting_period_hours
            || request.voting_duration_hours > self.parameters.max_voting_period_hours
        {
            return CreateProposalResponse {
                success: false,
                proposal_id: None,
                message: format!(
                    "Voting duration must be between {} and {} hours",
                    self.parameters.min_voting_period_hours,
                    self.parameters.max_voting_period_hours
                ),
            };
        }

        // Check creator's UT balance
        let tokenomics_manager = self.tokenomics_manager.read().await;
        let creator_ut_balance = tokenomics_manager
            .ut_holders
            .get(&request.creator)
            .map(|ut| ut.balance)
            .unwrap_or(0);

        if creator_ut_balance < self.parameters.min_ut_to_create_proposal {
            return CreateProposalResponse {
                success: false,
                proposal_id: None,
                message: format!(
                    "Insufficient UT balance. Required: {}, Current: {}",
                    self.parameters.min_ut_to_create_proposal,
                    creator_ut_balance
                ),
            };
        }

        // Create proposal
        let proposal_id = format!("PROP_{}", Utc::now().timestamp());
        let now = Utc::now();
        let voting_end = now + chrono::Duration::hours(request.voting_duration_hours as i64);

        let proposal = Proposal {
            proposal_id: proposal_id.clone(),
            title: request.title,
            description: request.description,
            proposal_type: request.proposal_type,
            creator: request.creator,
            created_at: now,
            voting_start: now,
            voting_end,
            status: ProposalStatus::Active,
            execution_data: None,
            results: None,
        };

        self.proposals.insert(proposal_id.clone(), proposal);
        self.votes.insert(proposal_id.clone(), Vec::new());

        CreateProposalResponse {
            success: true,
            proposal_id: Some(proposal_id),
            message: "Proposal created successfully".to_string(),
        }
    }

    /// Cast a vote
    pub async fn cast_vote(&mut self, request: VoteRequest) -> VoteResponse {
        // Check if proposal exists and is active
        let proposal = match self.proposals.get(&request.proposal_id) {
            Some(proposal) => proposal,
            None => {
                return VoteResponse {
                    success: false,
                    vote_id: None,
                    voting_power: None,
                    message: "Proposal not found".to_string(),
                };
            }
        };

        if proposal.status != ProposalStatus::Active {
            return VoteResponse {
                success: false,
                vote_id: None,
                voting_power: None,
                message: "Proposal is not active for voting".to_string(),
            };
        }

        // Check if voting period is still active
        if Utc::now() > proposal.voting_end {
            return VoteResponse {
                success: false,
                vote_id: None,
                voting_power: None,
                message: "Voting period has ended".to_string(),
            };
        }

        // Check voter's UT balance
        let tokenomics_manager = self.tokenomics_manager.read().await;
        let voter_ut_balance = tokenomics_manager
            .ut_holders
            .get(&request.voter)
            .map(|ut| ut.balance)
            .unwrap_or(0);

        if voter_ut_balance < self.parameters.min_ut_to_vote {
            return VoteResponse {
                success: false,
                vote_id: None,
                voting_power: None,
                message: format!(
                    "Insufficient UT balance to vote. Required: {}, Current: {}",
                    self.parameters.min_ut_to_vote,
                    voter_ut_balance
                ),
            };
        }

        // Check if user already voted
        if let Some(existing_votes) = self.votes.get(&request.proposal_id) {
            if existing_votes.iter().any(|vote| vote.voter == request.voter) {
                return VoteResponse {
                    success: false,
                    vote_id: None,
                    voting_power: None,
                    message: "User has already voted on this proposal".to_string(),
                };
            }
        }

        // Create vote
        let vote_id = format!("VOTE_{}_{}", request.proposal_id, Utc::now().timestamp());
        let vote = Vote {
            vote_id: vote_id.clone(),
            proposal_id: request.proposal_id.clone(),
            voter: request.voter.clone(),
            choice: request.choice,
            voting_power: voter_ut_balance,
            timestamp: Utc::now(),
            transaction_hash: None,
        };

        // Add vote
        self.votes
            .get_mut(&request.proposal_id)
            .unwrap()
            .push(vote);

        VoteResponse {
            success: true,
            vote_id: Some(vote_id),
            voting_power: Some(voter_ut_balance),
            message: "Vote cast successfully".to_string(),
        }
    }

    /// Finalize proposal results
    pub async fn finalize_proposal(&mut self, proposal_id: &str) -> Result<(), String> {
        let proposal = match self.proposals.get_mut(proposal_id) {
            Some(proposal) => proposal,
            None => return Err("Proposal not found".to_string()),
        };

        if proposal.status != ProposalStatus::Active {
            return Err("Proposal is not active".to_string());
        }

        if Utc::now() <= proposal.voting_end {
            return Err("Voting period has not ended yet".to_string());
        }

        // Calculate results
        let empty_votes = Vec::new();
        let votes = self.votes.get(proposal_id).unwrap_or(&empty_votes);
        let total_votes = votes.len() as u32;
        let yes_votes = votes.iter().filter(|v| v.choice == VoteChoice::Yes).count() as u32;
        let no_votes = votes.iter().filter(|v| v.choice == VoteChoice::No).count() as u32;
        let abstain_votes = votes.iter().filter(|v| v.choice == VoteChoice::Abstain).count() as u32;

        let total_voting_power: u128 = votes.iter().map(|v| v.voting_power).sum();
        let yes_voting_power: u128 = votes
            .iter()
            .filter(|v| v.choice == VoteChoice::Yes)
            .map(|v| v.voting_power)
            .sum();
        let no_voting_power: u128 = votes
            .iter()
            .filter(|v| v.choice == VoteChoice::No)
            .map(|v| v.voting_power)
            .sum();
        let abstain_voting_power: u128 = votes
            .iter()
            .filter(|v| v.choice == VoteChoice::Abstain)
            .map(|v| v.voting_power)
            .sum();

        // Check quorum
        let tokenomics_manager = self.tokenomics_manager.read().await;
        let total_ut_supply: u128 = tokenomics_manager.ut_holders.values().map(|ut| ut.balance).sum();
        let quorum_threshold = (total_ut_supply * self.parameters.quorum_threshold_percent as u128) / 100;
        let quorum_reached = total_voting_power >= quorum_threshold;

        // Check majority
        let majority_threshold = (total_voting_power * self.parameters.majority_threshold_percent as u128) / 100;
        let majority_achieved = yes_voting_power > majority_threshold;

        let results = ProposalResults {
            total_votes,
            yes_votes,
            no_votes,
            abstain_votes,
            total_voting_power,
            yes_voting_power,
            no_voting_power,
            abstain_voting_power,
            quorum_reached,
            majority_achieved,
        };

        // Update proposal status
        proposal.results = Some(results);
        proposal.status = if quorum_reached && majority_achieved {
            ProposalStatus::Passed
        } else {
            ProposalStatus::Rejected
        };

        Ok(())
    }

    /// Execute a passed proposal
    pub async fn execute_proposal(&mut self, proposal_id: &str, executor: String) -> Result<(), String> {
        // First, get the proposal and check if it can be executed
        let (proposal_type, _voting_end, _execution_delay_hours) = {
            let proposal = match self.proposals.get(proposal_id) {
                Some(proposal) => proposal,
                None => return Err("Proposal not found".to_string()),
            };

            if proposal.status != ProposalStatus::Passed {
                return Err("Proposal has not passed".to_string());
            }

            // Check execution delay
            if let Some(_results) = &proposal.results {
                let execution_time = proposal.voting_end + chrono::Duration::hours(self.parameters.execution_delay_hours as i64);
                if Utc::now() < execution_time {
                    return Err("Execution delay period has not passed".to_string());
                }
            }

            (proposal.proposal_type.clone(), proposal.voting_end, self.parameters.execution_delay_hours)
        };

        // Execute proposal based on type
        match &proposal_type {
            ProposalType::TokenomicsChange { parameter, new_value, .. } => {
                self.execute_tokenomics_change(parameter, new_value).await?;
            }
            ProposalType::TreasuryAllocation { recipient, amount, purpose } => {
                self.execute_treasury_allocation(recipient, *amount, purpose).await?;
            }
            ProposalType::ProtocolUpgrade { version, description } => {
                self.execute_protocol_upgrade(version, description).await?;
            }
            ProposalType::EmergencyAction { action, reason } => {
                self.execute_emergency_action(action, reason).await?;
            }
            ProposalType::General { category } => {
                self.execute_general_proposal(category).await?;
            }
        }

        // Update proposal status
        if let Some(proposal) = self.proposals.get_mut(proposal_id) {
            proposal.status = ProposalStatus::Executed;
            proposal.execution_data = Some(ExecutionData {
                executed_at: Some(Utc::now()),
                executor: Some(executor),
                transaction_hash: Some(format!("0x{}", hex::encode(proposal_id.as_bytes()))),
                result: Some("Successfully executed".to_string()),
            });
        }

        Ok(())
    }

    /// Execute tokenomics change
    async fn execute_tokenomics_change(&self, parameter: &str, new_value: &str) -> Result<(), String> {
        // In a real implementation, this would update the tokenomics configuration
        println!("Executing tokenomics change: {} = {}", parameter, new_value);
        Ok(())
    }

    /// Execute treasury allocation
    async fn execute_treasury_allocation(&self, recipient: &str, amount: u128, purpose: &str) -> Result<(), String> {
        // In a real implementation, this would transfer tokens from treasury
        println!("Executing treasury allocation: {} THP to {} for {}", amount, recipient, purpose);
        Ok(())
    }

    /// Execute protocol upgrade
    async fn execute_protocol_upgrade(&self, version: &str, description: &str) -> Result<(), String> {
        // In a real implementation, this would trigger a protocol upgrade
        println!("Executing protocol upgrade to version {}: {}", version, description);
        Ok(())
    }

    /// Execute emergency action
    async fn execute_emergency_action(&self, action: &str, reason: &str) -> Result<(), String> {
        // In a real implementation, this would execute emergency measures
        println!("Executing emergency action: {} (reason: {})", action, reason);
        Ok(())
    }

    /// Execute general proposal
    async fn execute_general_proposal(&self, category: &str) -> Result<(), String> {
        // In a real implementation, this would execute the general proposal
        println!("Executing general proposal in category: {}", category);
        Ok(())
    }

    /// Get all proposals
    pub fn get_proposals(&self) -> Vec<&Proposal> {
        self.proposals.values().collect()
    }

    /// Get proposal by ID
    pub fn get_proposal(&self, proposal_id: &str) -> Option<&Proposal> {
        self.proposals.get(proposal_id)
    }

    /// Get votes for a proposal
    pub fn get_proposal_votes(&self, proposal_id: &str) -> Vec<&Vote> {
        self.votes.get(proposal_id).map(|votes| votes.iter().collect()).unwrap_or_default()
    }

    /// Get user's voting history
    pub fn get_user_voting_history(&self, user_id: &str) -> Vec<&Vote> {
        self.votes
            .values()
            .flatten()
            .filter(|vote| vote.voter == user_id)
            .collect()
    }

    /// Update governance parameters
    pub fn update_parameters(&mut self, new_parameters: GovernanceParameters) {
        self.parameters = new_parameters;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenomics_config::TokenomicsConfig;

    #[tokio::test]
    async fn test_create_proposal() {
        let tokenomics_config = TokenomicsConfig::default();
        let tokenomics_manager = Arc::new(RwLock::new(NewTokenomicsManager::new(tokenomics_config)));
        let mut dao = GovernanceDAO::new(tokenomics_manager, TokenomicsConfig::default());

        let request = CreateProposalRequest {
            title: "Test Proposal".to_string(),
            description: "This is a test proposal".to_string(),
            proposal_type: ProposalType::General {
                category: "test".to_string(),
            },
            voting_duration_hours: 24,
            creator: "user_001".to_string(),
        };

        let response = dao.create_proposal(request).await;
        assert!(response.success);
        assert!(response.proposal_id.is_some());
    }

    #[tokio::test]
    async fn test_cast_vote() {
        let tokenomics_config = TokenomicsConfig::default();
        let tokenomics_manager = Arc::new(RwLock::new(NewTokenomicsManager::new(tokenomics_config)));
        let mut dao = GovernanceDAO::new(tokenomics_manager, TokenomicsConfig::default());

        // First create a proposal
        let create_request = CreateProposalRequest {
            title: "Test Proposal".to_string(),
            description: "This is a test proposal".to_string(),
            proposal_type: ProposalType::General {
                category: "test".to_string(),
            },
            voting_duration_hours: 24,
            creator: "user_001".to_string(),
        };

        let create_response = dao.create_proposal(create_request).await;
        let proposal_id = create_response.proposal_id.unwrap();

        // Then vote on it
        let vote_request = VoteRequest {
            proposal_id: proposal_id.clone(),
            voter: "user_002".to_string(),
            choice: VoteChoice::Yes,
        };

        let vote_response = dao.cast_vote(vote_request).await;
        assert!(vote_response.success);
        assert!(vote_response.vote_id.is_some());
    }
}
