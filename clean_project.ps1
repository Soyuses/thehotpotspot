# Clean project for GitHub upload
Write-Host "Cleaning The Hot Pot Spot project" -ForegroundColor Green
Write-Host "=================================" -ForegroundColor Green

# Clean build artifacts
Write-Host "`nCleaning build artifacts..." -ForegroundColor Yellow
if (Test-Path "target") {
    Remove-Item -Recurse -Force "target"
    Write-Host "OK: Removed target directory" -ForegroundColor Green
} else {
    Write-Host "OK: No target directory found" -ForegroundColor Green
}

# Clean test output files
Write-Host "`nCleaning test output files..." -ForegroundColor Yellow
$testFiles = @(
    "test_output.txt",
    "test_output2.txt", 
    "test_results.txt",
    "test_results_final.txt",
    "test_results_fixed.txt",
    "test_results_fixed2.txt"
)

foreach ($file in $testFiles) {
    if (Test-Path $file) {
        Remove-Item $file
        Write-Host "OK: Removed $file" -ForegroundColor Green
    }
}

# Clean temporary files
Write-Host "`nCleaning temporary files..." -ForegroundColor Yellow
$tempFiles = @(
    "*.tmp",
    "*.log",
    "*.cache"
)

foreach ($pattern in $tempFiles) {
    $files = Get-ChildItem -Name $pattern -ErrorAction SilentlyContinue
    foreach ($file in $files) {
        Remove-Item $file
        Write-Host "OK: Removed $file" -ForegroundColor Green
    }
}

# Clean Cargo.lock (will be regenerated)
Write-Host "`nCleaning Cargo.lock..." -ForegroundColor Yellow
if (Test-Path "Cargo.lock") {
    Remove-Item "Cargo.lock"
    Write-Host "OK: Removed Cargo.lock" -ForegroundColor Green
} else {
    Write-Host "OK: No Cargo.lock found" -ForegroundColor Green
}

# Create .gitignore if it doesn't exist
Write-Host "`nCreating .gitignore..." -ForegroundColor Yellow
if (-not (Test-Path ".gitignore")) {
    $gitignoreContent = @"
# Rust
/target/
Cargo.lock

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Logs
*.log

# Test outputs
test_output*.txt
test_results*.txt

# Temporary files
*.tmp
*.cache
"@
    $gitignoreContent | Out-File -FilePath ".gitignore" -Encoding UTF8
    Write-Host "OK: Created .gitignore" -ForegroundColor Green
} else {
    Write-Host "OK: .gitignore already exists" -ForegroundColor Green
}

# Create README for GitHub
Write-Host "`nUpdating README for GitHub..." -ForegroundColor Yellow
$readmeContent = @"
# The Hot Pot Spot - Blockchain Restaurant Network

A comprehensive blockchain-based restaurant franchise network with video surveillance system, built in Rust.

## Features

- **Blockchain Core**: Proof of Stake consensus, tokenomics, smart contracts
- **Video Surveillance**: Real-time streaming to Twitch/YouTube with face anonymization
- **Franchise Network**: Multi-node restaurant network with IPFS storage
- **Web Interfaces**: HTML dashboards for owners, customers, and video management
- **Mobile App**: React Native app for customers and franchise owners
- **API Integration**: REST API for external system integration

## Quick Start

1. **Install Rust**: https://rustup.rs/
2. **Clone repository**: `git clone <repository-url>`
3. **Build project**: `cargo build`
4. **Run server**: `cargo run --bin blockchain_project`
5. **Access interfaces**:
   - Main page: http://127.0.0.1:8080/
   - Video management: http://127.0.0.1:8080/video_management_dashboard.html
   - API testing: http://127.0.0.1:8080/api_test_interface.html

## Architecture

- **Backend**: Rust with Tokio async runtime
- **Blockchain**: Custom PoS consensus with tokenomics
- **Video System**: Real-time processing with anonymization
- **Storage**: IPFS for decentralized data storage
- **Frontend**: HTML/JavaScript with responsive design

## Documentation

- [Video Surveillance System](VIDEO_SURVEILLANCE_REPORT.md)
- [Frontend-Backend Integration](FRONTEND_BACKEND_INTEGRATION_REPORT.md)
- [Quick Start Guide](VIDEO_SYSTEM_QUICK_START.md)

## Testing

Run the test suite:
```bash
# PowerShell
.\simple_test.ps1
.\test_video_api.ps1

# Or manually
cargo test
cargo run --bin blockchain_project
```

## Legal Compliance

The system is designed to comply with Georgian Personal Data Protection Law and GDPR requirements for video surveillance and data processing.

## License

See [LICENSE](LICENSE) file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## Support

For questions and support, please open an issue in the GitHub repository.
"@

$readmeContent | Out-File -FilePath "README.md" -Encoding UTF8 -Force
Write-Host "OK: Updated README.md" -ForegroundColor Green

# Final report
Write-Host "`nCLEANING COMPLETED" -ForegroundColor Cyan
Write-Host "==================" -ForegroundColor Cyan
Write-Host "Project is now clean and ready for GitHub upload!" -ForegroundColor Green
Write-Host "`nNext steps:" -ForegroundColor Yellow
Write-Host "1. Initialize git repository: git init" -ForegroundColor White
Write-Host "2. Add files: git add ." -ForegroundColor White
Write-Host "3. Commit: git commit -m 'Initial commit'" -ForegroundColor White
Write-Host "4. Add remote: git remote add origin <repository-url>" -ForegroundColor White
Write-Host "5. Push: git push -u origin main" -ForegroundColor White
