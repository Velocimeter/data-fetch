use ethers::types::U64;
use sea_orm::Database;
use std::env;
use std::sync::Arc;
use tracing::instrument;

mod config;
mod types;
mod writer_options_data;

use config::{BASE_GAUGE, BASE_O_TOKEN, FANTOM_GAUGE, FANTOM_O_TOKEN, MULTICALL_ADDRESS};
use types::{Chain, ChainData};
use writer_options_data::write_options_data;

#[instrument]
pub async fn syncer() {
    let db_url = env::var("DATABASE_URL").expect("Should be defined in .env");

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

    for chain in chains {
        let pool = Arc::clone(&conn);
        tokio::spawn(async move {
            write_options_data(chain, pool).await.unwrap();
        });
    }
}
