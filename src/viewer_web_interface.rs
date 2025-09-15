use std::sync::Arc;
use tokio::sync::RwLock;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::viewer_arm::{ViewerARM, ViewerLoginRequest, UTActivityRequest, KYCRegistrationRequest};
use crate::tokenomics_config::TokenomicsConfig;
use crate::kyc_aml::KYCAmlManager;

/// Viewer web interface state
#[derive(Clone)]
pub struct ViewerWebState {
    pub viewer_arm: Arc<RwLock<ViewerARM>>,
}

/// Query parameters for viewer login
#[derive(Deserialize)]
pub struct ViewerLoginQuery {
    pub nickname: String,
    pub platform: String,
    pub phone: Option<String>,
}

/// Query parameters for UT activity
#[derive(Deserialize)]
pub struct UTActivityQuery {
    pub session_id: String,
    pub activity_type: String,
    pub reference: String,
    pub duration_minutes: Option<u32>,
    pub count: Option<u32>,
}

/// Create viewer web interface router
pub fn create_viewer_router() -> Router<ViewerWebState> {
    Router::new()
        .route("/", get(viewer_dashboard))
        .route("/login", post(viewer_login))
        .route("/activity", post(record_ut_activity))
        .route("/register", post(register_for_kyc))
        .route("/stats/:session_id", get(get_viewer_stats))
        .route("/leaderboard", get(get_ut_leaderboard))
        .route("/conversion-rounds", get(get_conversion_rounds))
}

/// Viewer dashboard page
async fn viewer_dashboard() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>The Hot Pot Spot - Viewer ARM</title>
    <style>
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
            background: white;
            border-radius: 15px;
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
            overflow: hidden;
        }
        .header {
            background: linear-gradient(135deg, #ff6b6b, #ee5a24);
            color: white;
            padding: 30px;
            text-align: center;
        }
        .header h1 {
            margin: 0;
            font-size: 2.5em;
            font-weight: 300;
        }
        .header p {
            margin: 10px 0 0 0;
            opacity: 0.9;
            font-size: 1.1em;
        }
        .content {
            padding: 40px;
        }
        .section {
            margin-bottom: 40px;
            padding: 30px;
            border: 1px solid #e0e0e0;
            border-radius: 10px;
            background: #fafafa;
        }
        .section h2 {
            color: #333;
            margin-top: 0;
            border-bottom: 2px solid #ff6b6b;
            padding-bottom: 10px;
        }
        .form-group {
            margin-bottom: 20px;
        }
        .form-group label {
            display: block;
            margin-bottom: 5px;
            font-weight: 600;
            color: #555;
        }
        .form-group input, .form-group select {
            width: 100%;
            padding: 12px;
            border: 1px solid #ddd;
            border-radius: 5px;
            font-size: 16px;
            box-sizing: border-box;
        }
        .btn {
            background: linear-gradient(135deg, #ff6b6b, #ee5a24);
            color: white;
            padding: 12px 30px;
            border: none;
            border-radius: 5px;
            cursor: pointer;
            font-size: 16px;
            font-weight: 600;
            transition: transform 0.2s;
        }
        .btn:hover {
            transform: translateY(-2px);
        }
        .btn-secondary {
            background: linear-gradient(135deg, #667eea, #764ba2);
        }
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
            margin-top: 20px;
        }
        .stat-card {
            background: white;
            padding: 20px;
            border-radius: 10px;
            box-shadow: 0 5px 15px rgba(0,0,0,0.1);
            text-align: center;
        }
        .stat-card h3 {
            margin: 0 0 10px 0;
            color: #ff6b6b;
            font-size: 2em;
        }
        .stat-card p {
            margin: 0;
            color: #666;
        }
        .platform-badge {
            display: inline-block;
            padding: 5px 15px;
            background: #667eea;
            color: white;
            border-radius: 20px;
            font-size: 0.9em;
            margin: 5px;
        }
        .status-pending { color: #f39c12; }
        .status-verified { color: #27ae60; }
        .status-rejected { color: #e74c3c; }
        .hidden { display: none; }
        .success-message {
            background: #d4edda;
            color: #155724;
            padding: 15px;
            border-radius: 5px;
            margin: 20px 0;
        }
        .error-message {
            background: #f8d7da;
            color: #721c24;
            padding: 15px;
            border-radius: 5px;
            margin: 20px 0;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🍔 The Hot Pot Spot</h1>
            <p>Viewer ARM - Manage Your UT Tokens & Earn THP</p>
        </div>
        
        <div class="content">
            <!-- Login Section -->
            <div class="section">
                <h2>🎮 Login as Viewer</h2>
                <form id="loginForm">
                    <div class="form-group">
                        <label for="nickname">Nickname:</label>
                        <input type="text" id="nickname" name="nickname" placeholder="Enter your streaming nickname" required>
                    </div>
                    <div class="form-group">
                        <label for="platform">Platform:</label>
                        <select id="platform" name="platform" required>
                            <option value="">Select platform</option>
                            <option value="twitch">Twitch</option>
                            <option value="youtube">YouTube</option>
                            <option value="facebook">Facebook Gaming</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label for="phone">Phone (optional, for KYC):</label>
                        <input type="tel" id="phone" name="phone" placeholder="+995 XXX XXX XXX">
                    </div>
                    <button type="submit" class="btn">Login</button>
                </form>
                <div id="loginResult"></div>
            </div>

            <!-- UT Activity Section -->
            <div class="section hidden" id="activitySection">
                <h2>🎯 Record UT Activity</h2>
                <form id="activityForm">
                    <div class="form-group">
                        <label for="activityType">Activity Type:</label>
                        <select id="activityType" name="activity_type" required>
                            <option value="">Select activity</option>
                            <option value="streaming">Streaming (10 UT/min)</option>
                            <option value="comment">Comment (5 UT)</option>
                            <option value="share">Share (20 UT)</option>
                            <option value="like">Like (2 UT)</option>
                            <option value="view">View (1 UT)</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label for="reference">Reference:</label>
                        <input type="text" id="reference" name="reference" placeholder="Stream ID, comment ID, etc." required>
                    </div>
                    <div class="form-group hidden" id="durationGroup">
                        <label for="durationMinutes">Duration (minutes):</label>
                        <input type="number" id="durationMinutes" name="duration_minutes" min="1" max="120">
                    </div>
                    <div class="form-group hidden" id="countGroup">
                        <label for="count">Count:</label>
                        <input type="number" id="count" name="count" min="1" value="1">
                    </div>
                    <button type="submit" class="btn">Record Activity</button>
                </form>
                <div id="activityResult"></div>
            </div>

            <!-- KYC Registration Section -->
            <div class="section hidden" id="kycSection">
                <h2>📋 Register for KYC (Earn THP Tokens)</h2>
                <form id="kycForm">
                    <div class="form-group">
                        <label for="fullName">Full Name:</label>
                        <input type="text" id="fullName" name="full_name" required>
                    </div>
                    <div class="form-group">
                        <label for="email">Email:</label>
                        <input type="email" id="email" name="email" required>
                    </div>
                    <div class="form-group">
                        <label for="kycPhone">Phone:</label>
                        <input type="tel" id="kycPhone" name="phone" required>
                    </div>
                    <div class="form-group">
                        <label for="tshirtSize">T-shirt Size:</label>
                        <select id="tshirtSize" name="tshirt_size" required>
                            <option value="">Select size</option>
                            <option value="XXS">XXS</option>
                            <option value="XS">XS</option>
                            <option value="S">S</option>
                            <option value="M">M</option>
                            <option value="L">L</option>
                            <option value="XL">XL</option>
                            <option value="XXL">XXL</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label for="favoriteDish">Favorite Dish:</label>
                        <input type="text" id="favoriteDish" name="favorite_dish" placeholder="e.g., Plov, Khinkali, etc." required>
                    </div>
                    <div class="form-group">
                        <label for="password">Password:</label>
                        <input type="password" id="password" name="password" required>
                    </div>
                    <div class="form-group">
                        <label for="qrCode">QR Code from Check (optional):</label>
                        <input type="text" id="qrCode" name="qr_code" placeholder="Scan QR code to link wallet">
                    </div>
                    <button type="submit" class="btn btn-secondary">Register for KYC</button>
                </form>
                <div id="kycResult"></div>
            </div>

            <!-- Statistics Section -->
            <div class="section hidden" id="statsSection">
                <h2>📊 Your Statistics</h2>
                <div class="stats-grid">
                    <div class="stat-card">
                        <h3 id="utBalance">0</h3>
                        <p>UT Tokens</p>
                    </div>
                    <div class="stat-card">
                        <h3 id="stBalance">0</h3>
                        <p>THP Tokens</p>
                    </div>
                    <div class="stat-card">
                        <h3 id="streamingTime">0</h3>
                        <p>Minutes Streamed</p>
                    </div>
                    <div class="stat-card">
                        <h3 id="totalUTEarned">0</h3>
                        <p>Total UT Earned</p>
                    </div>
                </div>
            </div>

            <!-- Leaderboard Section -->
            <div class="section">
                <h2>🏆 UT Leaderboard</h2>
                <div id="leaderboard"></div>
            </div>

            <!-- Conversion Rounds Section -->
            <div class="section">
                <h2>🔄 Conversion Rounds</h2>
                <div id="conversionRounds"></div>
            </div>
        </div>
    </div>

    <script>
        let currentSessionId = null;
        let currentUserStats = null;

        // Show/hide form fields based on activity type
        document.getElementById('activityType').addEventListener('change', function() {
            const durationGroup = document.getElementById('durationGroup');
            const countGroup = document.getElementById('countGroup');
            
            if (this.value === 'streaming') {
                durationGroup.classList.remove('hidden');
                countGroup.classList.add('hidden');
            } else {
                durationGroup.classList.add('hidden');
                countGroup.classList.remove('hidden');
            }
        });

        // Login form
        document.getElementById('loginForm').addEventListener('submit', async function(e) {
            e.preventDefault();
            
            const formData = new FormData(this);
            const params = new URLSearchParams();
            for (let [key, value] of formData.entries()) {
                params.append(key, value);
            }
            
            try {
                const response = await fetch('/login?' + params.toString(), {
                    method: 'POST'
                });
                const result = await response.json();
                
                const resultDiv = document.getElementById('loginResult');
                if (result.success) {
                    currentSessionId = result.session_id;
                    resultDiv.innerHTML = `<div class="success-message">Login successful! Session: ${result.session_id}</div>`;
                    
                    // Show other sections
                    document.getElementById('activitySection').classList.remove('hidden');
                    document.getElementById('kycSection').classList.remove('hidden');
                    document.getElementById('statsSection').classList.remove('hidden');
                    
                    // Update stats
                    updateStats(result);
                } else {
                    resultDiv.innerHTML = `<div class="error-message">Login failed: ${result.message}</div>`;
                }
            } catch (error) {
                document.getElementById('loginResult').innerHTML = `<div class="error-message">Error: ${error.message}</div>`;
            }
        });

        // Activity form
        document.getElementById('activityForm').addEventListener('submit', async function(e) {
            e.preventDefault();
            
            if (!currentSessionId) {
                document.getElementById('activityResult').innerHTML = '<div class="error-message">Please login first</div>';
                return;
            }
            
            const formData = new FormData(this);
            const activityData = {
                session_id: currentSessionId,
                activity_type: formData.get('activity_type'),
                reference: formData.get('reference'),
                duration_minutes: formData.get('duration_minutes') ? parseInt(formData.get('duration_minutes')) : null,
                count: formData.get('count') ? parseInt(formData.get('count')) : null
            };
            
            try {
                const response = await fetch('/activity', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify(activityData)
                });
                const result = await response.json();
                
                const resultDiv = document.getElementById('activityResult');
                if (result.success) {
                    resultDiv.innerHTML = `<div class="success-message">Activity recorded! Earned ${result.ut_earned} UT. New balance: ${result.new_ut_balance} UT</div>`;
                    
                    // Update stats
                    if (currentUserStats) {
                        currentUserStats.ut_balance = result.new_ut_balance;
                        updateStatsDisplay(currentUserStats);
                    }
                } else {
                    resultDiv.innerHTML = `<div class="error-message">Failed to record activity: ${result.message}</div>`;
                }
            } catch (error) {
                document.getElementById('activityResult').innerHTML = `<div class="error-message">Error: ${error.message}</div>`;
            }
        });

        // KYC form
        document.getElementById('kycForm').addEventListener('submit', async function(e) {
            e.preventDefault();
            
            if (!currentSessionId) {
                document.getElementById('kycResult').innerHTML = '<div class="error-message">Please login first</div>';
                return;
            }
            
            const formData = new FormData(this);
            const kycData = {
                session_id: currentSessionId,
                full_name: formData.get('full_name'),
                email: formData.get('email'),
                phone: formData.get('phone'),
                tshirt_size: formData.get('tshirt_size'),
                favorite_dish: formData.get('favorite_dish'),
                password: formData.get('password'),
                qr_code: formData.get('qr_code') || null
            };
            
            try {
                const response = await fetch('/register', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify(kycData)
                });
                const result = await response.json();
                
                const resultDiv = document.getElementById('kycResult');
                if (result.success) {
                    resultDiv.innerHTML = `<div class="success-message">Registration successful! User ID: ${result.user_id}. KYC status: ${result.kyc_status}</div>`;
                } else {
                    resultDiv.innerHTML = `<div class="error-message">Registration failed: ${result.message}</div>`;
                }
            } catch (error) {
                document.getElementById('kycResult').innerHTML = `<div class="error-message">Error: ${error.message}</div>`;
            }
        });

        function updateStats(loginResult) {
            currentUserStats = {
                ut_balance: loginResult.ut_balance || 0,
                st_balance: loginResult.st_balance || 0,
                kyc_status: loginResult.kyc_status
            };
            updateStatsDisplay(currentUserStats);
        }

        function updateStatsDisplay(stats) {
            document.getElementById('utBalance').textContent = stats.ut_balance;
            document.getElementById('stBalance').textContent = stats.st_balance || 0;
            document.getElementById('streamingTime').textContent = stats.total_streaming_time || 0;
            document.getElementById('totalUTEarned').textContent = stats.total_ut_earned || 0;
        }

        // Load leaderboard and conversion rounds on page load
        async function loadLeaderboard() {
            try {
                const response = await fetch('/leaderboard');
                const leaderboard = await response.json();
                
                const leaderboardDiv = document.getElementById('leaderboard');
                leaderboardDiv.innerHTML = leaderboard.map((entry, index) => `
                    <div class="stat-card">
                        <h3>#${index + 1}</h3>
                        <p>${entry[0]}: ${entry[1]} UT</p>
                    </div>
                `).join('');
            } catch (error) {
                document.getElementById('leaderboard').innerHTML = '<div class="error-message">Failed to load leaderboard</div>';
            }
        }

        async function loadConversionRounds() {
            try {
                const response = await fetch('/conversion-rounds');
                const rounds = await response.json();
                
                const roundsDiv = document.getElementById('conversionRounds');
                roundsDiv.innerHTML = rounds.map(round => `
                    <div class="stat-card">
                        <h3>${round.round_id}</h3>
                        <p>Pool: ${round.total_pool} THP</p>
                        <p>Status: ${round.status}</p>
                    </div>
                `).join('');
            } catch (error) {
                document.getElementById('conversionRounds').innerHTML = '<div class="error-message">Failed to load conversion rounds</div>';
            }
        }

        // Load data on page load
        loadLeaderboard();
        loadConversionRounds();
    </script>
</body>
</html>
    "#)
}

/// Handle viewer login
async fn viewer_login(
    State(state): State<ViewerWebState>,
    Query(params): Query<ViewerLoginQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let login_request = ViewerLoginRequest {
        nickname: params.nickname,
        platform: params.platform,
        phone: params.phone,
    };

    let mut viewer_arm = state.viewer_arm.write().await;
    let response = viewer_arm.login_viewer(login_request).await;

    Ok(Json(serde_json::to_value(response).unwrap()))
}

/// Handle UT activity recording
async fn record_ut_activity(
    State(state): State<ViewerWebState>,
    Json(request): Json<UTActivityRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut viewer_arm = state.viewer_arm.write().await;
    let response = viewer_arm.record_ut_activity(request).await;

    Ok(Json(serde_json::to_value(response).unwrap()))
}

/// Handle KYC registration
async fn register_for_kyc(
    State(state): State<ViewerWebState>,
    Json(request): Json<KYCRegistrationRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut viewer_arm = state.viewer_arm.write().await;
    let response = viewer_arm.register_for_kyc(request).await;

    Ok(Json(serde_json::to_value(response).unwrap()))
}

/// Get viewer statistics
async fn get_viewer_stats(
    State(state): State<ViewerWebState>,
    Path(session_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let viewer_arm = state.viewer_arm.read().await;
    let stats = viewer_arm.get_viewer_stats(&session_id).await;

    match stats {
        Some(stats) => Ok(Json(serde_json::to_value(stats).unwrap())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Get UT leaderboard
async fn get_ut_leaderboard(
    State(state): State<ViewerWebState>,
) -> Result<Json<Vec<(String, u128)>>, StatusCode> {
    let viewer_arm = state.viewer_arm.read().await;
    let leaderboard = viewer_arm.get_ut_leaderboard(10).await;

    Ok(Json(leaderboard))
}

/// Get conversion rounds
async fn get_conversion_rounds(
    State(state): State<ViewerWebState>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let viewer_arm = state.viewer_arm.read().await;
    let rounds = viewer_arm.get_conversion_rounds().await;

    Ok(Json(rounds.into_iter().map(|r| serde_json::to_value(r).unwrap()).collect()))
}
