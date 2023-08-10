use super::Chain;
use ethers::{
    abi::Address,
    prelude::*,
    utils::{format_units, parse_units, to_checksum},
};
use eyre::Result;
use futures::future::try_join_all;
use std::{fs::File, io::Write, sync::Arc};
use tokio::time::{sleep, Duration, Instant};

#[derive(Debug, Default, EthEvent, PartialEq, Eq)]
#[ethevent(name = "Boosted", abi = "Boosted(uint256,uint256,address)")]
struct BoostedEvent {
    #[ethevent(indexed)]
    pub timestamp: U256,
    pub total_locked: U256,
    pub locker: Address,
}

pub async fn write_logs_data(chain: Chain) -> Result<()> {
    let provider = Provider::<Http>::try_from(chain.get_chain_data().rpc_url.to_string())?;
    let client = Arc::new(provider);
    let contract_address = chain.get_chain_data().booster_address.parse::<Address>()?;

    let mut ranges: Vec<(U64, U64)> = Vec::new();
    let current_block = &client.get_block_number().await?;
    let current_block = *current_block;

    let mut i = chain.get_chain_data().from_block;
    while i <= current_block {
        let range_end = std::cmp::min(i + 1_024 - 1, current_block);
        ranges.push((i, range_end));
        i += U64::from(1_024);
    }

    let mut ranges_chunks = ranges.chunks(5);
    let mut logs = Vec::new();

    let mut start_time = Instant::now();

    while let Some(chunk) = ranges_chunks.next() {
        let mut tasks = Vec::with_capacity(chunk.len());
        for range in chunk {
            let (from, to) = range.clone();
            let client = Arc::clone(&client);

            tasks.push(tokio::spawn(async move {
                get_logs(client, contract_address, from, to).await
            }))
        }
        let results = try_join_all(tasks).await?;
        for result in results {
            logs.push(result?)
        }

        let elapsed = Instant::now() - start_time;
        if elapsed > Duration::from_secs(5) {
            sleep(Duration::from_secs(5)).await;
            start_time = Instant::now();
        }
    }

    let flattened_results = logs.iter().flatten().collect::<Vec<&Log>>();
    println!("Total buys: {}", flattened_results.len());
    let path = format!("logs_{}.csv", chain.get_chain_data().id);
    let file = File::create(path)?;

    let mut total_boster_used = 0.0;
    for log in flattened_results {
        let event = parse_log::<BoostedEvent>(log.clone());
        match event {
            Ok(event) => {
                let total_locked = format_units(event.total_locked, "ether")
                    .unwrap()
                    .parse::<f64>()
                    .unwrap();
                let boosted = total_locked - (total_locked / chain.get_chain_data().match_rate);
                let airdrop = boosted / chain.get_chain_data().booster_total
                    * chain.get_chain_data().aidrop_total;
                total_boster_used += boosted;
                writeln!(
                    &file,
                    "{},{},{},{},{}",
                    event.timestamp,
                    boosted,
                    airdrop,
                    to_checksum(&event.locker, None),
                    parse_units(airdrop, "ether").unwrap(),
                )?;
            }
            Err(e) => panic!("Error parsing log: {:?}", e),
        }
    }

    println!(
        "Booster used: {}%",
        total_boster_used / chain.get_chain_data().booster_total * 100.0
    );
    Ok(())
}

async fn get_logs(
    client: Arc<Provider<Http>>,
    contract_address: Address,
    from: U64,
    to: U64,
) -> Result<Vec<Log>, ProviderError> {
    client
        .get_logs(
            &Filter::new()
                .address(contract_address)
                .event("Boosted(uint256,uint256,address)")
                .from_block(from)
                .to_block(to),
        )
        .await
}
