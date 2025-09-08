# Simple project test
Write-Host "Testing The Hot Pot Spot project" -ForegroundColor Green
Write-Host "=================================" -ForegroundColor Green

# Check compilation
Write-Host "`nChecking compilation..." -ForegroundColor Yellow
cargo check
if ($LASTEXITCODE -eq 0) {
    Write-Host "OK: Compilation successful" -ForegroundColor Green
} else {
    Write-Host "ERROR: Compilation failed" -ForegroundColor Red
    exit 1
}

# Check build
Write-Host "`nChecking build..." -ForegroundColor Yellow
cargo build --bin blockchain_project
if ($LASTEXITCODE -eq 0) {
    Write-Host "OK: Build successful" -ForegroundColor Green
} else {
    Write-Host "ERROR: Build failed" -ForegroundColor Red
    exit 1
}

# Check required files
Write-Host "`nChecking required files..." -ForegroundColor Yellow

$requiredFiles = @(
    "src/main.rs",
    "src/lib.rs",
    "src/video_surveillance.rs",
    "src/video_api.rs",
    "src/streaming_integration.rs",
    "src/enhanced_web_server.rs",
    "video_management_dashboard.html",
    "video_consent_interface.html",
    "api_test_interface.html",
    "Cargo.toml",
    "README.md"
)

$missingFiles = @()
foreach ($file in $requiredFiles) {
    if (Test-Path $file) {
        Write-Host "OK: $file" -ForegroundColor Green
    } else {
        Write-Host "MISSING: $file" -ForegroundColor Red
        $missingFiles += $file
    }
}

# Check documentation
Write-Host "`nChecking documentation..." -ForegroundColor Yellow
$docFiles = @(
    "README.md",
    "VIDEO_SURVEILLANCE_REPORT.md",
    "FRONTEND_BACKEND_INTEGRATION_REPORT.md",
    "VIDEO_SYSTEM_QUICK_START.md"
)

foreach ($doc in $docFiles) {
    if (Test-Path $doc) {
        $size = (Get-Item $doc).Length
        Write-Host "OK: $doc ($size bytes)" -ForegroundColor Green
    } else {
        Write-Host "MISSING: $doc" -ForegroundColor Red
    }
}

# Final report
Write-Host "`nFINAL REPORT" -ForegroundColor Cyan
Write-Host "============" -ForegroundColor Cyan

if ($missingFiles.Count -eq 0) {
    Write-Host "SUCCESS: All main files present" -ForegroundColor Green
} else {
    Write-Host "WARNING: $($missingFiles.Count) files missing" -ForegroundColor Yellow
}

Write-Host "`nProject ready for:" -ForegroundColor Green
Write-Host "  - Run: cargo run --bin blockchain_project" -ForegroundColor White
Write-Host "  - Test: .\test_video_api.ps1" -ForegroundColor White
Write-Host "  - GitHub upload" -ForegroundColor White

Write-Host "`nNext steps:" -ForegroundColor Cyan
Write-Host "  1. Clean test data" -ForegroundColor White
Write-Host "  2. Restart blockchain" -ForegroundColor White
Write-Host "  3. Prepare for GitHub upload" -ForegroundColor White

Write-Host "`nTesting completed!" -ForegroundColor Green
