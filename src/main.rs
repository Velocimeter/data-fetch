use ethers::{
    abi::Address,
    prelude::*,
    utils::{format_units, to_checksum},
};
use eyre::Result;
use futures::future::try_join_all;
use std::{fs::File, io::Write, sync::Arc};
use tokio::time::{sleep, Duration, Instant};

const FANTOM_RPC_URL: &str = "https://fantom.blockpi.network/v1/rpc/public";
const CANTO_RPC_URL: &str = "https://velocimeter.tr.zone/";
const PULSE_RPC_URL: &str = "https://rpc.pulsechain.com";

const FANTOM_BOOSTER_ADDRESS: &str = "0xbD777Af905F603797CFC1E8eBa229DaD26FE4863";
const CANTO_BOOSTER_ADDRESS: &str = "0xacBDa0a9AF99f391C5994Dd5b262E04c778eeBE7";
const PULSE_BOOSTER_ADDRESS: &str = "0xC5d4E462b96cC73283EB452B15147c17Af413313";

struct ChainData {
    id: u32,
    rpc_url: String,
    booster_address: String,
    from_block: U64,
}

enum Chain {
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

#[derive(Debug, Default, EthEvent, PartialEq, Eq)]
#[ethevent(name = "Boosted", abi = "Boosted(uint256,uint256,address)")]
struct BoostedEvent {
    #[ethevent(indexed)]
    pub timestamp: U256,
    pub total_locked: U256,
    pub locker: Address,
}

#[tokio::main]
async fn main() -> Result<()> {
    let fantom_chain = Chain::Fantom(ChainData {
        id: 250,
        rpc_url: FANTOM_RPC_URL.to_string(),
        booster_address: FANTOM_BOOSTER_ADDRESS.to_string(),
        from_block: U64::from(66450156),
    });
    let canto_chain = Chain::Canto(ChainData {
        id: 7700,
        rpc_url: CANTO_RPC_URL.to_string(),
        booster_address: CANTO_BOOSTER_ADDRESS.to_string(),
        from_block: U64::from(5313097),
    });
    let pulse_chain = Chain::Pulse(ChainData {
        id: 369,
        rpc_url: PULSE_RPC_URL.to_string(),
        booster_address: PULSE_BOOSTER_ADDRESS.to_string(),
        from_block: U64::from(17917480),
    });

    let chains = vec![fantom_chain, canto_chain, pulse_chain];

    for chain in chains {
        println!("Starting chain {}", chain.get_chain_data().id);
        write_logs_data(chain).await?;
    }

    Ok(())
}

async fn write_logs_data(chain: Chain) -> Result<()> {
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
    let time = Instant::now();

    while let Some(chunk) = ranges_chunks.next() {
        println!("Processing next chunk");
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
        println!("elapsed time is {}ms", elapsed.as_millis());
        if elapsed > Duration::from_secs(5) {
            println!("sleeping for 5 seconds");
            sleep(Duration::from_secs(5)).await;
            start_time = Instant::now();
        }
        let total_elapsed = Instant::now() - time;
        println!("Chunk processed, moving to next chunk");
        println!("Total elapsed time is {}ms", total_elapsed.as_millis());
    }

    println!("All tasks finished");

    let flattened_results = logs.iter().flatten().collect::<Vec<&Log>>();
    println!("Total logs: {}", flattened_results.len());
    let path = format!("logs_{}.csv", chain.get_chain_data().id);
    let file = File::create(path)?;

    for log in flattened_results {
        let event = parse_log::<BoostedEvent>(log.clone());
        match event {
            Ok(event) => {
                writeln!(
                    &file,
                    "{},{},{}",
                    event.timestamp,
                    format_units(event.total_locked, "ether").unwrap(),
                    to_checksum(&event.locker, None)
                )?;
            }
            Err(e) => panic!("Error parsing log: {:?}", e),
        }
    }

    Ok(())
}

async fn get_logs(
    client: Arc<Provider<Http>>,
    contract_address: Address,
    from: U64,
    to: U64,
) -> Result<Vec<Log>, ProviderError> {
    println!("Getting logs from {} to {}", from, to);
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
