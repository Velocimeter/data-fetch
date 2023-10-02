use ethers::types::U64;
use futures::future::join_all;
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, instrument};

mod config;
mod types;
mod writer_options_data;

use config::{BASE_GAUGE, BASE_O_TOKEN, FANTOM_GAUGE, FANTOM_O_TOKEN, MULTICALL_ADDRESS};
use types::{Chain, ChainData};
use writer_options_data::write_options_data;

#[instrument]
pub async fn syncer() {
    let db_url = env::var("DATABASE_URL").expect("Should be defined in .env");
    let iteration_time_secs = env::var("ITERATION_TIME_SECS")
        .expect("Should be defined in .env")
        .parse::<u64>()
        .expect("Should be a number");

    // set up connection
    let conn = Database::connect(db_url)
        .await
        .expect("Could not connect to database");

    let conn: Arc<_> = Arc::new(conn);

    let fantom_chain = Chain::Fantom(ChainData {
        id: 250,
        rpc_url: env::var("FANTOM_RPC_URL")
            .expect("Should be defined in .env")
            .to_string(),
        o_token_address: FANTOM_O_TOKEN.to_string(),
        native_gauge_address: FANTOM_GAUGE.to_string(),
        from_block: U64::from(64965262),
        rpc_step: 1_024,
        multicall_address: MULTICALL_ADDRESS.to_string(),
    });
    let base_chain = Chain::Base(ChainData {
        id: 8453,
        rpc_url: env::var("BASE_RPC_URL")
            .expect("Should be defined in .env")
            .to_string()
            .to_string(),
        o_token_address: BASE_O_TOKEN.to_string(),
        native_gauge_address: BASE_GAUGE.to_string(),
        from_block: U64::from(1963125),
        rpc_step: 1_024,
        multicall_address: MULTICALL_ADDRESS.to_string(),
    });
    // let mantle_chain = Chain::Mantle(ChainData {
    //     id: 5000,
    //     rpc_url: MANTLE_RPC_URL.to_string(),
    //     o_token_address: MANTLE_O_TOKEN.to_string(),
    //     native_gauge_address: MANTLE_GAUGE.to_string(),
    //     from_block: U64::from(51025),
    //     rpc_step: 1_024,
    //     multicall_address: MULTICALL_ADDRESS.to_string(),
    // });

    let chains = vec![
        fantom_chain,
        base_chain,
        //   mantle_chain
    ];

    iteration_run(chains.clone(), Arc::clone(&conn)).await;

    let six_hours = Duration::from_secs(iteration_time_secs);
    loop {
        info!("Sleeping for {} seconds", six_hours.as_secs());
        sleep(six_hours).await;
        iteration_run(chains.clone(), Arc::clone(&conn)).await;
    }
}

async fn iteration_run(chains: Vec<Chain>, conn: Arc<DatabaseConnection>) {
    let mut tasks = vec![];
    for chain in chains {
        let pool = Arc::clone(&conn);
        let task = tokio::spawn(async move {
            write_options_data(chain, pool).await.unwrap();
        });
        tasks.push(task);
    }
    join_all(tasks).await;
}
