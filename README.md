# SwapX - Decentralized Exchange on Substrate

A sophisticated DEX implementation using ink! smart contracts on Substrate, providing automated market making, liquidity provision, and token swapping capabilities.

## 🚀 Features

### Core Functionality
- 💱 Token swapping with customizable slippage protection
- 💧 Liquidity provision and removal
- 📊 Automated Market Making (AMM)
- 💰 Protocol fee mechanism for sustainability

### Technical Features
- ⚡ Optimized for minimal gas consumption
- 🔒 Security-first implementation
- ✨ Event emission for all major operations
- 🔄 Real-time price calculations

## 📋 Prerequisites

### Development Environment
```bash
# Install Rust & Substrate
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
cargo install cargo-contract
cargo install contracts-node

# Install Node.js dependencies
npm install
```

### Build & Deploy
```bash
# Build contract
cargo contract build

# Start local node
substrate-contracts-node --dev

# Deploy contract
cargo contract upload --suri //Alice
cargo contract instantiate --suri //Alice --args 1 <PROTOCOL_FUND_ADDRESS>
```

### Run Frontend
```bash
# Update contract address in app.js
# Then start the server
npm start
```

## Contract Interface

### Main Functions
- `swap_tokens(token_in, amount_in, token_out, min_amount_out)`
- `add_liquidity(token, amount)`
- `remove_liquidity(token, amount)`
- `get_swap_rate(token_in, token_out)`

### Events
- TokensSwapped
- LiquidityAdded
- LiquidityRemoved
- TokenDeposited

## Testing
```bash
cargo test
```

## Project Structure
```
swapx/
├── src/
│   └── lib.rs          # Smart contract implementation
├── app.js              # Frontend logic
├── index.html          # UI interface
└── package.json        # Dependencies
```

## License
MIT

## Contributing
1. Fork repository
2. Create feature branch
3. Commit changes
4. Push to branch
5. Submit pull request