/// Handles swap actions
use crate::Config;
use anyhow::{self, Context};
use bindings::{i_portfolio::IPortfolio, i_portfolio_actions::SwapCall, shared_types::Order};
use ethers::prelude::*;
use std::sync::Arc;

pub async fn main(cfg: &Config) -> Result<(), anyhow::Error> {
    let client = Arc::new({
        let ws_provider = Provider::<Ws>::connect(&cfg.rpc_url).await?;

        let chain_id = ws_provider.get_chainid().await?;
        let wallet = std::env::var("PRIVATE_KEY")
            .context("swap.rs: PRIVATE_KEY env var not set")?
            .parse::<LocalWallet>()?
            .with_chain_id(chain_id.as_u64());

        SignerMiddleware::new(ws_provider, wallet)
    });

    let portfolio = IPortfolio::new(cfg.portfolio_address.parse::<Address>()?, client);
    let version = portfolio.version().call().await?;
    println!("Portfolio version: {}", version);

    println!("swap");

    do_swap(SwapCall {
        args: Order {
            use_max: false,
            pool_id: 0,
            input: 0,
            output: 0,
            sell_asset: false,
        },
    })
    .await?;
    Ok(())
}

async fn parse_args() -> Result<SwapCall, anyhow::Error> {
    unimplemented!("not implemented yet")
}

/// Gracefully executes a swap transaction on Portfolio and propagates any errors.
async fn do_swap(args: SwapCall) -> Result<(), anyhow::Error> {
    println!("do swap");
    Ok(())
}
