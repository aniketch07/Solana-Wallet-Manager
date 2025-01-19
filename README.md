# Solana Wallet Manager

The Solana Wallet Manager is a Rust-based application designed to simplify the management of multiple Solana wallets. This tool allows users to generate new wallets, view wallet balances, and request airdrops to their wallets on the Solana Devnet.

## Features

- **Generate New Wallets**: Easily create new Solana wallets with secure key pairs.
- **View Wallet Balances**: Check the balance of each wallet in lamports.
- **Request Airdrops**: Request airdrops to any of your wallets directly from the application.
- **Manage Multiple Wallets**: Store and manage multiple wallets in a single JSON file.

## Usage

### Generate a New Wallet
- If no wallets are found, the application will automatically generate a new wallet and display the public key.

### View Wallet Balances
- If wallets are found, their balances will be listed.

### Request Airdrops
- The user can request an airdrop to any existing wallet by selecting the wallet number.

### Add New Wallets
- If the user decides not to request an airdrop, they will be prompted to generate a new wallet.

## Technical Details

- **Language**: Rust
- **Dependencies**: solana-client, solana-sdk, serde, serde_json

## Getting Started

### Clone the repository:
```sh
git clone https://github.com/yourusername/solana-wallet-manager.git
cd solana-wallet-manager
```
### Build the project:
```sh
cargo build
```
### Run the application:
```sh
cargo run
```
