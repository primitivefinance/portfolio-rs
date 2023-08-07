use colored::Colorize;

use super::Config;
use anyhow;
use ethers::prelude::*;

use std::sync::Arc;

use bindings::i_portfolio::{CreatePoolFilter, IPortfolio};

/// Lists the pools of a Portfolio contract.
pub async fn list_pools(cfg: &Config) -> Result<(), anyhow::Error> {
    let ws_provider = Provider::<Ws>::connect(&cfg.rpc_url).await?;
    let ws_client = Arc::new(ws_provider);
    let connected_msg = format!(
        "{} {} {} {} {}",
        "Connected".bright_green(),
        "\n   - RPC:",
        &cfg.rpc_url.bold().bright_magenta(),
        "\n   - Portfolio:",
        &cfg.portfolio_address.bold().bright_magenta()
    );
    println!("{}", connected_msg);

    let start_block = 3982259;
    let address = (&cfg.portfolio_address).parse::<Address>()?;
    let contract = IPortfolio::new(address, ws_client);

    let events: Vec<CreatePoolFilter> = contract
        .create_pool_filter()
        .from_block(start_block)
        .query()
        .await?;

    println!("{}{}", "Listing pools... please be patient".blue(), " 🤗");
    for (i, event) in events.into_iter().enumerate() {
        if i == 0 {
            println!(
                "{} {} {}",
                "Found".blue(),
                (i + 1).to_string().blue().bold(),
                "pools".blue()
            );
        }
        println!(
            "   - {}{} {} {}",
            "#".cyan(),
            i.to_string().bold().cyan(),
            "- id:".purple(),
            event.pool_id.to_string().bold().purple()
        );
    }

    // Print an exit message
    let exit_message = format!("{}{}", "Done listing pools".bright_green(), " 🤗");
    println!("{}", exit_message);
    Ok(())
}
