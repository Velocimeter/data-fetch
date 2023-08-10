mod airdrop_scipt;
mod timelock_txs_creator;

use std::io;

use dotenv::dotenv;
use ethers::{
    types::{U256, U64},
    utils::parse_units,
};
use eyre::Result;

use airdrop_scipt::write_logs_data;

const FANTOM_RPC_URL: &str = "https://fantom.blockpi.network/v1/rpc/public";
const CANTO_RPC_URL: &str = "https://velocimeter.tr.zone/";
const PULSE_RPC_URL: &str = "https://rpc.pulsechain.com";

const FANTOM_BOOSTER_ADDRESS: &str = "0xbD777Af905F603797CFC1E8eBa229DaD26FE4863";
const CANTO_BOOSTER_ADDRESS: &str = "0xacBDa0a9AF99f391C5994Dd5b262E04c778eeBE7";
const PULSE_BOOSTER_ADDRESS: &str = "0xC5d4E462b96cC73283EB452B15147c17Af413313";

pub struct ChainData {
    id: u32,
    rpc_url: String,
    booster_address: String,
    from_block: U64,
    booster_total: f64,
    aidrop_total: f64,
    match_rate: f64,
}

pub enum Chain {
    Fantom(ChainData),
    Canto(ChainData),
    Pulse(ChainData),
}

impl Chain {
    fn get_chain_data(&self) -> &ChainData {
        match self {
            Chain::Fantom(data) => data,
            Chain::Canto(data) => data,
            Chain::Pulse(data) => data,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    println!("Input operation type:");
    println!("1 for airdrop script, 2 for timelock tx creation");

    let mut operation = String::new();

    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read operation");

    let operation: u32 = operation.trim().parse().expect("Please type a number!");

    match operation {
        1 => {
            let fantom_chain = Chain::Fantom(ChainData {
                id: 250,
                rpc_url: FANTOM_RPC_URL.to_string(),
                booster_address: FANTOM_BOOSTER_ADDRESS.to_string(),
                from_block: U64::from(66450156),
                booster_total: 5263.0,
                aidrop_total: 210000.0,
                match_rate: 1.02,
            });
            let canto_chain = Chain::Canto(ChainData {
                id: 7700,
                rpc_url: CANTO_RPC_URL.to_string(),
                booster_address: CANTO_BOOSTER_ADDRESS.to_string(),
                from_block: U64::from(5313097),
                booster_total: 1000.0,
                aidrop_total: 210000.0,
                match_rate: 1.01,
            });
            let pulse_chain = Chain::Pulse(ChainData {
                id: 369,
                rpc_url: PULSE_RPC_URL.to_string(),
                booster_address: PULSE_BOOSTER_ADDRESS.to_string(),
                from_block: U64::from(17917480),
                booster_total: 2359213.0,
                aidrop_total: 120000.0,
                match_rate: 1.01,
            });

            let chains = vec![fantom_chain, canto_chain, pulse_chain];

            for chain in chains {
                println!("Chain {}", chain.get_chain_data().id);
                write_logs_data(chain).await?;
            }
        }
        2 => {
            println!("Input amount to transfer:");

            let mut amount = String::new();

            io::stdin()
                .read_line(&mut amount)
                .expect("Failed to read amount");

            let amount: f64 = amount.trim().parse().expect("Type a floating number!");
            let pu = parse_units(amount.to_string(), "ether").unwrap();
            let transfer_amount = U256::from(pu);
            timelock_txs_creator::create_tx(transfer_amount).await?;
        }
        _ => {
            println!("Invalid operation type");
            return Ok(());
        }
    }

    Ok(())
}
