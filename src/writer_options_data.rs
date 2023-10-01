// use crate::o_token_datas::average_lock_time;
use super::{internal_error, Chain};
use axum::http::StatusCode;
use ethers::{abi::Address, contract::Multicall, prelude::*};
use eyre::Result;
use futures::future::try_join_all;
use sea_orm::{sea_query, ActiveValue, DatabaseConnection, EntityTrait};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, Duration, Instant};
use tracing::{info, instrument};

use rust_velocimeter_data::bindings::{ExerciseLpFilter, Gauge};
use rust_velocimeter_data::database::o_token_datas::{
    ActiveModel as OTokenData, Column as OTokenDatasColumn, Entity as OTokenDatas,
};

#[instrument(skip(chain, conn))]
pub async fn write_options_data(chain: Chain, conn: Arc<DatabaseConnection>) -> Result<()> {
    info!(
        "Writing options data for chain: {}",
        chain.get_chain_data().id
    );
    let provider = Provider::<Http>::try_from(chain.get_chain_data().rpc_url.to_string())?;
    let client = Arc::new(provider);

    let mut multicall = Multicall::<Provider<Http>>::new(
        client.clone(),
        Some(
            chain
                .get_chain_data()
                .multicall_address
                .parse::<Address>()
                .expect("Address is set by hand"),
        ),
    )
    .await?;

    let contract_address = chain.get_chain_data().o_token_address.parse::<Address>()?;
    let mut ranges: Vec<(U64, U64)> = Vec::new();
    let current_block = &client.get_block_number().await?;
    let current_block = *current_block;

    let mut i = chain.get_chain_data().from_block;
    let chain_step = chain.get_chain_data().rpc_step;
    while i <= current_block {
        let range_end = std::cmp::min(i + chain_step - 1, current_block);
        ranges.push((i, range_end));
        i += U64::from(chain_step);
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
                get_exerciselp_logs(client, contract_address, from, to).await
            }))
        }
        let results = try_join_all(tasks).await?;
        for result in results {
            logs.push(match result {
                Ok(logs) => logs,
                Err(e) => panic!("Error getting logs: {:?}", e),
            })
        }

        let elapsed = Instant::now() - start_time;
        if elapsed > Duration::from_secs(5) {
            sleep(Duration::from_secs(5)).await;
            start_time = Instant::now();
        }
    }

    let flattened_results = logs.iter().flatten().collect::<Vec<&Log>>();
    let mut exercisors = Vec::<H160>::new();

    for log in flattened_results {
        let event = parse_log::<ExerciseLpFilter>(log.clone());
        match event {
            Ok(event) => {
                let sender = event.sender;
                if !exercisors.contains(&sender) {
                    exercisors.push(sender);
                }
            }
            Err(e) => panic!("Error parsing log: {:?}", e),
        }
    }

    multicall.add_calls(
        false,
        exercisors.iter().map(|addy| {
            let gauge_addy = chain
                .get_chain_data()
                .native_gauge_address
                .parse::<Address>()
                .unwrap();
            let gauge = Gauge::new(gauge_addy, client.clone());
            gauge.lock_end(*addy)
        }),
    );

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let results = multicall.call_array::<U256>().await?;

    let results = results.iter().map(|lock_end| {
        let lock_end = lock_end.as_u64();
        let lock_end = i64::try_from(lock_end).unwrap();
        let days_until_lock_end: i64 =
            (lock_end - i64::try_from(now).expect("Should safely convert")) / 86400;
        if days_until_lock_end < 0 {
            0
        } else {
            days_until_lock_end
        }
    });

    let average_lock_time = results.sum::<i64>() / i64::try_from(exercisors.len()).unwrap();

    info!("New average lock is {}", average_lock_time);

    let new_o_token_data = OTokenData {
        chain_id: ActiveValue::Unchanged(chain.get_chain_data().id),
        average_lock_time: ActiveValue::Set(average_lock_time as f64),
        w_native_rewarded: ActiveValue::Set(0.0),
    };

    let _ = write_average_lock(conn, new_o_token_data).await;

    Ok(())
}

async fn get_exerciselp_logs(
    client: Arc<Provider<Http>>,
    contract_address: Address,
    from: U64,
    to: U64,
) -> Result<Vec<Log>, ProviderError> {
    client
        .get_logs(
            &Filter::new()
                .address(contract_address)
                .event("ExerciseLp(address,address,uint256,uint256,uint256)")
                .from_block(from)
                .to_block(to),
        )
        .await
}

#[instrument]
async fn write_average_lock(
    conn: Arc<DatabaseConnection>,
    o_token_data: OTokenData,
) -> Result<(), StatusCode> {
    info!("Writing average lock time to DB");
    info!(
        "Writing average lock time to DB and it is: {:?}",
        o_token_data.average_lock_time
    );
    match OTokenDatas::insert(o_token_data)
        .on_conflict(
            sea_query::OnConflict::column(OTokenDatasColumn::ChainId)
                .update_column(OTokenDatasColumn::AverageLockTime)
                .update_column(OTokenDatasColumn::WNativeRewarded)
                .to_owned(),
        )
        .exec(conn.as_ref())
        .await
        .map_err(internal_error)
    {
        Ok(_) => {}
        Err(e) => {
            info!("Error writing to DB: {:?}", e);
            return Err(e);
        }
    }
    info!("DB write successful");
    Ok(())
}
