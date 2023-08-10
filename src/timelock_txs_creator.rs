use ethers::contract::abigen;
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Address, U256};
use eyre::Result;
use std::env;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;

abigen!(Timelock, "./src/abis/Timelock.json");
abigen!(ERC20, "./src/abis/ERC20.json");

const RPC_URL: &str = "https://base.blockpi.network/v1/rpc/public";
const TIMELOCK_ADDRESS: &str = "0xa36b73043ded64586aaf28d3a70fa9a20bc514fc";
const BVM_ADDRESS: &str = "0xd386a121991E51Eab5e3433Bf5B1cF4C8884b47a";
const BASE_MSIG_ADDRESS: &str = "0xfA89A4C7F79Dc4111c116a0f01061F4a7D9fAb73";
const DELAY: u64 = 86400;

pub async fn create_tx(transfer_amount: U256) -> Result<()> {
    let pk = env::var("PRIVATE_KEY").expect("Should be defined in .env");

    let client = Arc::new({
        // connect to the network
        let provider = Provider::<Http>::try_from(RPC_URL)?;
        let chain_id = provider.get_chainid().await?;

        // this wallet's private key
        let wallet = pk.parse::<LocalWallet>()?.with_chain_id(chain_id.as_u64());

        SignerMiddleware::new(provider, wallet)
    });
    let timelock = Timelock::new(parse_hardcoded_addy(TIMELOCK_ADDRESS), client.clone());
    let bvm = ERC20::new(
        BVM_ADDRESS
            .parse::<Address>()
            .expect("Address is hardcoded"),
        client.clone(),
    );

    let transfer_tx = bvm
        .transfer(parse_hardcoded_addy(BASE_MSIG_ADDRESS), transfer_amount)
        .tx;
    let data = transfer_tx.data();
    let predecessor = [0; 32];
    let salt = [0; 32];

    let schedule = timelock.schedule(
        parse_hardcoded_addy(BVM_ADDRESS),
        U256::from(0),
        data.unwrap().clone(),
        predecessor,
        salt,
        U256::from(DELAY),
    );

    schedule.send().await?.await?.expect("Timelock tx failed");

    let schedule_tx = schedule.tx;

    let schedule_data = schedule_tx.data().unwrap();

    let path = format!("tx_creator__amount_{}", transfer_amount.to_string());
    let file = File::create(path)?;
    writeln!(&file, "{}", schedule_data)?;

    println!("Tx: {:?}", schedule_data);

    Ok(())
}

fn parse_hardcoded_addy(addy: &str) -> Address {
    addy.parse::<Address>().expect("Address is hardcoded")
}
