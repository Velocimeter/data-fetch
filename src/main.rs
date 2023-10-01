use axum::{extract::Path, http::StatusCode, response::Json, routing::get, Extension, Router};
use ethers::types::U64;
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use std::{net::SocketAddr, sync::Arc};
use tracing::{info, instrument};

use dotenv::dotenv;
use std::env;

mod writer_options_data;
use writer_options_data::write_options_data;

use rust_velocimeter_data::database::o_token_datas::Entity as OTokenDatas;
use rust_velocimeter_data::database::o_token_datas::Model as OTokenData;

// const MANTLE_RPC_URL: &str = "https://rpc.mantle.xyz/";

const FANTOM_O_TOKEN: &str = "0xF9EDdca6B1e548B0EC8cDDEc131464F462b8310D";
const BASE_O_TOKEN: &str = "0x762eb51D2e779EeEc9B239FFB0B2eC8262848f3E";
// const MANTLE_O_TOKEN: &str = "0x3b19B8EC75BBf85848d133F1a47710EeEd57Bd90";

const FANTOM_GAUGE: &str = "0xa3643a5d5b672a267199227cd3e95ed0b41dbd52";
const BASE_GAUGE: &str = "0x3f5129112754d4fbe7ab228c2d5e312b2bc79a06";
// const MANTLE_GAUGE: &str = "0x43072ee491c57de81971fb804e0ea5f4836a073d";

const MULTICALL_ADDRESS: &str = "0xcA11bde05977b3631167028862bE2a173976CA11";

#[derive(Debug)]
pub struct ChainData {
    id: i32,
    rpc_url: String,
    o_token_address: String,
    native_gauge_address: String,
    from_block: U64,
    rpc_step: i32,
    multicall_address: String,
}

#[derive(Debug)]
pub enum Chain {
    Fantom(ChainData),
    Base(ChainData),
    Mantle(ChainData),
}

impl Chain {
    fn get_chain_data(&self) -> &ChainData {
        match self {
            Chain::Fantom(data) => data,
            Chain::Base(data) => data,
            Chain::Mantle(data) => data,
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt().init();

    info!("Tracing initialized");

    tokio::spawn(syncer());

    server().await;
}

async fn server() {
    let db_url = env::var("DATABASE_URL").expect("Should be defined in .env");

    // set up connection
    let conn = Database::connect(db_url)
        .await
        .expect("Could not connect to database");

    // build our application with some routes
    let app = Router::new()
        .route("/", get(root))
        .route("/options-data/:chain_id", get(give_options_data))
        .layer(Extension(conn));

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[instrument]
async fn syncer() {
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

#[axum_macros::debug_handler]
async fn give_options_data(
    Path(chain_id): Path<i32>,
    Extension(conn): Extension<DatabaseConnection>,
) -> Result<Json<OTokenData>, StatusCode> {
    info!("chain_id: {}", chain_id);
    let res = OTokenDatas::find_by_id(chain_id)
        .one(&conn)
        .await
        .map_err(internal_error)?;

    match res {
        Some(data) => Ok(Json(data.into())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(_err: E) -> StatusCode
where
    E: std::error::Error,
{
    StatusCode::INTERNAL_SERVER_ERROR
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
