use super::Config;
use bindings::i_portfolio_struct::{IPortfolioStruct, PortfolioPool};
use colored::Colorize;

use ethers::{
    prelude::*,
    utils::{format_ether, parse_ether},
};

pub async fn main(cfg: &Config, pool_id: &str) -> Result<(), anyhow::Error> {
    let _ = cfg;
    let start_info_msg = format!(
        "{} {} {} {}
        ",
        "Getting info for pool".yellow(),
        pool_id.bold().magenta(),
        "please be patient...".yellow(),
        " 🤗"
    );
    println!("{}", start_info_msg.on_black());

    let ws_provider = Provider::<Ws>::connect(&cfg.rpc_url).await?;
    let client = std::sync::Arc::new(ws_provider);

    let pool_id = pool_id.parse::<u64>().unwrap();

    let portfolio =
        IPortfolioStruct::new(cfg.portfolio_address.parse::<Address>()?, client.clone());

    let pool: PortfolioPool = portfolio.pools(pool_id).call().await?;

    let decoded = PoolId(pool_id).decode();

    let spot_price: U256 = bindings::i_portfolio::IPortfolio::new(
        cfg.portfolio_address.parse::<Address>()?,
        client.clone(),
    )
    .get_spot_price(pool_id)
    .call()
    .await?;
    // Message to println at end
    // Header
    // Pool Info
    // id: {pool_id}
    // For each attribute, print on new line with colored value
    let pool_info_msg = format!(
        "
        {}
        - id: {}
        {}
        - mark price: {}
        {}
        - altered?: {}
        - controlled?: {}
        - pair nonce: {}
        - pool nonce: {}
        - reserve x: {}
        - reserve y: {}
        - liquidity: {}
        - controller: {}
        - strategy: {}
        - fee bps: {}
        - priority fee bps: {}",
        "Pool Info:".yellow().bold(),
        pool_id.to_string().yellow(),
        "Economic Info:".yellow().bold(),
        format_ether(spot_price).to_string().yellow(),
        "Pool State:".yellow().bold(),
        decoded.0.to_string().yellow(),
        decoded.1.to_string().yellow(),
        decoded.2.to_string().yellow(),
        decoded.3.to_string().yellow(),
        format_ether(pool.virtual_x).to_string().yellow(),
        format_ether(pool.virtual_y).to_string().yellow(),
        format_ether(pool.liquidity).to_string().yellow(),
        pool.controller.to_string().yellow(),
        pool.strategy.to_string().yellow(),
        pool.fee_basis_points.to_string().yellow(),
        pool.priority_fee_basis_points.to_string().yellow(),
    );
    println!("{}", pool_info_msg.on_black());

    Ok(())
}

/// Implements useful methods for pool-ids.
struct PoolId(u64);

impl PoolId {
    /// Decodes the key information embedded into the poolId.
    /// Pool ids are 64-bits, with the following information encoded:
    /// - 0-4 bits: altered? bool
    /// - 4-8 bits: controlled? bool
    /// - 8-32 bits: pair nonce u24
    /// - 32-64 bits: pool nonce u32
    fn decode(&self) -> (bool, bool, u32, u32) {
        let altered = self.0 & 0b0000_0000_0000_0000_0000_0000_0000_0001 != 0;
        let controlled = self.0 & 0b0000_0000_0000_0000_0000_0000_0000_0010 != 0;
        let pair_nonce = (self.0 & 0b0000_0000_0000_0000_0000_0011_1111_1100) >> 2;
        let pool_nonce = (self.0 & 0b1111_1111_1111_1111_1111_1100_0000_0000) >> 10;

        (altered, controlled, pair_nonce as u32, pool_nonce as u32)
    }
}
