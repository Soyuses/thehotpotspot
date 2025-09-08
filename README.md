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
2. **Clone repository**: git clone <repository-url>
3. **Build project**: cargo build
4. **Run server**: cargo run --bin blockchain_project
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
`ash
# PowerShell
.\simple_test.ps1
.\test_video_api.ps1

# Or manually
cargo test
cargo run --bin blockchain_project
`

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
