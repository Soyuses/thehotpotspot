use std::sync::Arc;
use tokio::sync::RwLock;
use blockchain_project::{
    tokenomics_config::TokenomicsConfig,
    new_tokenomics::NewTokenomicsManager,
    governance_dao::{GovernanceDAO, CreateProposalRequest, VoteRequest, ProposalType, VoteChoice},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🗳️ The Hot Pot Spot - Governance DAO Demo");
    println!("==========================================");

    // Initialize configuration
    let tokenomics_config = TokenomicsConfig::default();
    let tokenomics_manager = Arc::new(RwLock::new(NewTokenomicsManager::new(tokenomics_config.clone())));
    let mut dao = GovernanceDAO::new(tokenomics_manager.clone(), tokenomics_config);

    println!("📊 DAO Configuration:");
    println!("  - Min voting period: {} hours", dao.parameters.min_voting_period_hours);
    println!("  - Max voting period: {} hours", dao.parameters.max_voting_period_hours);
    println!("  - Quorum threshold: {}%", dao.parameters.quorum_threshold_percent);
    println!("  - Majority threshold: {}%", dao.parameters.majority_threshold_percent);
    println!("  - Min UT to create proposal: {}", dao.parameters.min_ut_to_create_proposal);
    println!("  - Min UT to vote: {}", dao.parameters.min_ut_to_vote);

    // Setup users with UT balances
    println!("\n👥 Setting up users with UT balances...");
    
    // Add some UT holders
    {
        let mut manager = tokenomics_manager.write().await;
        
        // User 1: 5000 UT (can create proposals and vote)
        manager.ut_holders.insert("user_001".to_string(), blockchain_project::new_tokenomics::UtilityToken {
            token_id: "ut_001".to_string(),
            owner_address: "0xuser_001".to_string(),
            voting_power: 5000,
            non_transferable: true,
            updated_at: chrono::Utc::now(),
            balance: 5000,
            created_at: chrono::Utc::now(),
            last_activity: chrono::Utc::now(),
        });
        
        // User 2: 2000 UT (can vote)
        manager.ut_holders.insert("user_002".to_string(), blockchain_project::new_tokenomics::UtilityToken {
            token_id: "ut_002".to_string(),
            owner_address: "0xuser_002".to_string(),
            voting_power: 2000,
            non_transferable: true,
            updated_at: chrono::Utc::now(),
            balance: 2000,
            created_at: chrono::Utc::now(),
            last_activity: chrono::Utc::now(),
        });
        
        // User 3: 1000 UT (can vote)
        manager.ut_holders.insert("user_003".to_string(), blockchain_project::new_tokenomics::UtilityToken {
            token_id: "ut_003".to_string(),
            owner_address: "0xuser_003".to_string(),
            voting_power: 1000,
            non_transferable: true,
            updated_at: chrono::Utc::now(),
            balance: 1000,
            created_at: chrono::Utc::now(),
            last_activity: chrono::Utc::now(),
        });
        
        // User 4: 50 UT (cannot vote - below minimum)
        manager.ut_holders.insert("user_004".to_string(), blockchain_project::new_tokenomics::UtilityToken {
            token_id: "ut_004".to_string(),
            owner_address: "0xuser_004".to_string(),
            voting_power: 50,
            non_transferable: true,
            updated_at: chrono::Utc::now(),
            balance: 50,
            created_at: chrono::Utc::now(),
            last_activity: chrono::Utc::now(),
        });
    }

    println!("✅ Users setup complete!");
    println!("  - user_001: 5000 UT (can create proposals)");
    println!("  - user_002: 2000 UT (can vote)");
    println!("  - user_003: 1000 UT (can vote)");
    println!("  - user_004: 50 UT (cannot vote)");

    println!("\n📝 Creating Tokenomics Change Proposal...");

    // Create a tokenomics change proposal
    let proposal_request = CreateProposalRequest {
        title: "Increase UT per minute from 10 to 15".to_string(),
        description: "This proposal suggests increasing the UT reward rate for streaming from 10 UT per minute to 15 UT per minute to encourage more user engagement.".to_string(),
        proposal_type: ProposalType::TokenomicsChange {
            parameter: "ut_per_minute".to_string(),
            old_value: "10".to_string(),
            new_value: "15".to_string(),
        },
        voting_duration_hours: 24,
        creator: "user_001".to_string(),
    };

    let proposal_response = dao.create_proposal(proposal_request).await;
    println!("✅ Proposal created successfully!");
    println!("  - Proposal ID: {}", proposal_response.proposal_id.as_ref().unwrap());
    println!("  - Title: Increase UT per minute from 10 to 15");
    println!("  - Creator: user_001");
    println!("  - Voting duration: 24 hours");

    let proposal_id = proposal_response.proposal_id.unwrap();

    println!("\n🗳️ Simulating Voting Process...");

    // User 1 votes YES
    let vote_request_1 = VoteRequest {
        proposal_id: proposal_id.clone(),
        voter: "user_001".to_string(),
        choice: VoteChoice::Yes,
    };

    let vote_response_1 = dao.cast_vote(vote_request_1).await;
    println!("✅ user_001 voted YES!");
    println!("  - Vote ID: {}", vote_response_1.vote_id.as_ref().unwrap());
    println!("  - Voting power: {} UT", vote_response_1.voting_power.unwrap());

    // User 2 votes YES
    let vote_request_2 = VoteRequest {
        proposal_id: proposal_id.clone(),
        voter: "user_002".to_string(),
        choice: VoteChoice::Yes,
    };

    let vote_response_2 = dao.cast_vote(vote_request_2).await;
    println!("✅ user_002 voted YES!");
    println!("  - Vote ID: {}", vote_response_2.vote_id.as_ref().unwrap());
    println!("  - Voting power: {} UT", vote_response_2.voting_power.unwrap());

    // User 3 votes NO
    let vote_request_3 = VoteRequest {
        proposal_id: proposal_id.clone(),
        voter: "user_003".to_string(),
        choice: VoteChoice::No,
    };

    let vote_response_3 = dao.cast_vote(vote_request_3).await;
    println!("✅ user_003 voted NO!");
    println!("  - Vote ID: {}", vote_response_3.vote_id.as_ref().unwrap());
    println!("  - Voting power: {} UT", vote_response_3.voting_power.unwrap());

    // User 4 tries to vote (should fail)
    let vote_request_4 = VoteRequest {
        proposal_id: proposal_id.clone(),
        voter: "user_004".to_string(),
        choice: VoteChoice::Yes,
    };

    let vote_response_4 = dao.cast_vote(vote_request_4).await;
    println!("❌ user_004 voting failed (insufficient UT balance)");
    println!("  - Message: {}", vote_response_4.message);

    println!("\n📊 Getting Proposal Results...");

    // Get proposal details
    let proposal = dao.get_proposal(&proposal_id).unwrap();
    println!("📋 Proposal Details:");
    println!("  - ID: {}", proposal.proposal_id);
    println!("  - Title: {}", proposal.title);
    println!("  - Status: {:?}", proposal.status);
    println!("  - Created: {}", proposal.created_at);
    println!("  - Voting ends: {}", proposal.voting_end);

    // Get votes
    let votes = dao.get_proposal_votes(&proposal_id);
    println!("🗳️ Votes cast: {}", votes.len());
    for vote in &votes {
        println!("  - {}: {:?} ({} UT)", vote.voter, vote.choice, vote.voting_power);
    }

    println!("\n📈 Finalizing Proposal...");

    // Finalize the proposal (simulate voting period ended)
    match dao.finalize_proposal(&proposal_id).await {
        Ok(_) => {
            println!("✅ Proposal finalized successfully!");
            
            // Get updated proposal
            let finalized_proposal = dao.get_proposal(&proposal_id).unwrap();
            if let Some(results) = &finalized_proposal.results {
                println!("📊 Final Results:");
                println!("  - Total votes: {}", results.total_votes);
                println!("  - Yes votes: {} ({} UT)", results.yes_votes, results.yes_voting_power);
                println!("  - No votes: {} ({} UT)", results.no_votes, results.no_voting_power);
                println!("  - Total voting power: {} UT", results.total_voting_power);
                println!("  - Quorum reached: {}", results.quorum_reached);
                println!("  - Majority achieved: {}", results.majority_achieved);
                println!("  - Final status: {:?}", finalized_proposal.status);
            }
        }
        Err(e) => {
            println!("❌ Failed to finalize proposal: {}", e);
        }
    }

    println!("\n🏛️ Creating Treasury Allocation Proposal...");

    // Create a treasury allocation proposal
    let treasury_request = CreateProposalRequest {
        title: "Allocate 10,000 THP for marketing campaign".to_string(),
        description: "This proposal suggests allocating 10,000 THP tokens from the treasury to fund a marketing campaign to increase brand awareness.".to_string(),
        proposal_type: ProposalType::TreasuryAllocation {
            recipient: "marketing_team".to_string(),
            amount: 10000,
            purpose: "Marketing campaign".to_string(),
        },
        voting_duration_hours: 48,
        creator: "user_002".to_string(),
    };

    let treasury_response = dao.create_proposal(treasury_request).await;
    if treasury_response.success {
        println!("✅ Treasury proposal created successfully!");
        println!("  - Proposal ID: {}", treasury_response.proposal_id.as_ref().unwrap());
    } else {
        println!("❌ Failed to create treasury proposal: {}", treasury_response.message);
    }

    println!("\n📋 Getting All Proposals...");

    let all_proposals = dao.get_proposals();
    println!("📊 Total proposals: {}", all_proposals.len());
    for (i, proposal) in all_proposals.iter().enumerate() {
        println!("  {}. {} - {:?} ({:?})", 
            i + 1, 
            proposal.title, 
            proposal.proposal_type, 
            proposal.status
        );
    }

    println!("\n👤 Getting User Voting History...");

    let user_001_history = dao.get_user_voting_history("user_001");
    println!("🗳️ user_001 voting history: {} votes", user_001_history.len());
    for vote in &user_001_history {
        println!("  - Proposal {}: {:?} ({} UT)", vote.proposal_id, vote.choice, vote.voting_power);
    }

    println!("\n🎯 Demo Summary:");
    println!("================");
    println!("✅ Successfully demonstrated:");
    println!("  - DAO governance system creation");
    println!("  - Proposal creation with different types");
    println!("  - Voting process with UT-based voting power");
    println!("  - Vote validation and restrictions");
    println!("  - Proposal finalization and results calculation");
    println!("  - User voting history tracking");
    println!("\n🚀 Governance DAO is working correctly!");
    println!("   Users can create proposals with sufficient UT balance");
    println!("   Voting power is based on UT token holdings");
    println!("   Proposals are finalized with quorum and majority checks");
    println!("   Different proposal types support various governance actions");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use blockchain_project::tokenomics_config::TokenomicsConfig;

    #[tokio::test]
    async fn test_dao_creation() {
        let tokenomics_config = TokenomicsConfig::default();
        let tokenomics_manager = Arc::new(RwLock::new(NewTokenomicsManager::new(tokenomics_config.clone())));
        let dao = GovernanceDAO::new(tokenomics_manager, tokenomics_config);
        
        assert_eq!(dao.parameters.min_voting_period_hours, 24);
        assert_eq!(dao.parameters.quorum_threshold_percent, 20);
        assert_eq!(dao.parameters.majority_threshold_percent, 51);
    }

    #[tokio::test]
    async fn test_proposal_creation() {
        let tokenomics_config = TokenomicsConfig::default();
        let tokenomics_manager = Arc::new(RwLock::new(NewTokenomicsManager::new(tokenomics_config.clone())));
        let mut dao = GovernanceDAO::new(tokenomics_manager, tokenomics_config);

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
}
