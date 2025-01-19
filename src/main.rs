use std::{fs::File, io::{self, Write, Read}, path::Path};

use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct WalletInfo {
    public_key: String,
    secret_key: String,
}

const RPC_URL: &str="https://api.devnet.solana.com";
fn main() {
    let mut wallets = read_wallets_from_file();
    if wallets.is_empty() {
        println!("No wallets found. Generating a new one...");
        let wallet_info = generate_new_wallet();
        wallets.push(wallet_info);
        save_wallets_to_file(&wallets);
        println!("Generated new wallet with public key: {}", wallets.last().unwrap().public_key);
    } else {
        list_wallets_with_balances(&wallets);
        println!("Would you like to request an airdrop to an existing wallet? (y/n)");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read line");

        if buffer.trim() == "y" {
            println!("Select the wallet number to request airdrop:");
            for (i, wallet) in wallets.iter().enumerate() {
                println!("{}: {}", i, wallet.public_key);
            }

            buffer.clear();
            io::stdin().read_line(&mut buffer).expect("Failed to read line");
            if let Ok(index) = buffer.trim().parse::<usize>() {
                if index < wallets.len() {
                    let public_key = wallets[index].public_key.parse::<Pubkey>().expect("Invalid public key");
                    request_airdrop(&public_key);
                } else {
                    println!("Invalid wallet number.");
                }
            } else {
                println!("Invalid input.");
            }
        } else {
            println!("Would you like to generate a new wallet? (y/n)");
            buffer.clear();
            io::stdin().read_line(&mut buffer).expect("Failed to read line");

            if buffer.trim() == "y" {
                let wallet_info = generate_new_wallet();
                wallets.push(wallet_info);
                save_wallets_to_file(&wallets);
                println!("Generated new wallet with public key: {}", wallets.last().unwrap().public_key);
            }
        }
    }
}

fn generate_new_wallet() -> WalletInfo {
    let keypair = Keypair::new();
    let public_key = keypair.pubkey();
    let secret_key = keypair.to_base58_string();

    WalletInfo {
        public_key: public_key.to_string(),
        secret_key,
    }
}

fn save_wallets_to_file(wallets: &[WalletInfo]) {
    let mut file = File::create("solana_wallets.json").expect("Unable to create file");
    let wallets_json = serde_json::to_string_pretty(wallets).expect("Unable to serialize wallet info");
    file.write_all(wallets_json.as_bytes()).expect("Unable to write data");
    println!("Wallets have been saved to solana_wallets.json");
}

fn read_wallets_from_file() -> Vec<WalletInfo> {
    let path = Path::new("solana_wallets.json");
    if path.exists() {
        let mut file = File::open(path).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read file");
        serde_json::from_str(&contents).expect("Unable to parse JSON")
    } else {
        Vec::new()
    }
}

fn list_wallets_with_balances(wallets: &[WalletInfo]) {
    let rpc_url = RPC_URL;
    let client = RpcClient::new(rpc_url.to_string());

    for wallet in wallets {
        let public_key = wallet.public_key.parse::<Pubkey>().expect("Invalid public key");
        match client.get_balance(&public_key) {
            Ok(balance) => {
                println!("Wallet: {}, Balance: {} lamports", wallet.public_key, balance);
            }
            Err(err) => {
                println!("Failed to get balance for wallet {}: {}", wallet.public_key, err);
            }
        }
    }
}

fn request_airdrop(key: &Pubkey) {
    let rpc_url = RPC_URL;
    let client = RpcClient::new(rpc_url.to_string());
    let airdrop_amount = 1_000_000_000; // Amount of lamports to airdrop (1 SOL = 1,000,000,000 lamports)

    match client.request_airdrop(key, airdrop_amount) {
        Ok(signature) => {
            println!("Airdrop requested successfully. Signature: {}", signature);
        }
        Err(err) => {
            println!("Airdrop request failed: {}", err);
        }
    }
}