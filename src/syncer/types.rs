use ethers::types::U64;

#[derive(Debug)]
pub struct ChainData {
    pub id: i32,
    pub rpc_url: String,
    pub o_token_address: String,
    pub native_gauge_address: String,
    pub from_block: U64,
    pub rpc_step: i32,
    pub multicall_address: String,
}

#[derive(Debug)]
pub enum Chain {
    Fantom(ChainData),
    Base(ChainData),
    // Mantle(ChainData),
}

impl Chain {
    pub fn get_chain_data(&self) -> &ChainData {
        match self {
            Chain::Fantom(data) => data,
            Chain::Base(data) => data,
            // Chain::Mantle(data) => data,
        }
    }
}
