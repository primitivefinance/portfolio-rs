use colored::Colorize;

use super::{ActArgs, App, Commands::Act, Config};
use crate::invoke;
use anyhow;
use clap::{Arg, Command};
use ethers::prelude::*;

use inquire::{formatter::MultiOptionFormatter, MultiSelect};
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

    let mut pool_ids = Vec::<u64>::new();
    for (i, event) in events.into_iter().enumerate() {
        if i == 0 {
            println!(
                "{} {} {}",
                "Found".blue(),
                (i + 1).to_string().blue().bold(),
                "pools".blue()
            );
        }

        pool_ids.push(event.pool_id);

        println!(
            "   - {}{} {} {}",
            "#".cyan(),
            i.to_string().bold().cyan(),
            "- id:".purple(),
            event.pool_id.to_string().bold().purple()
        );
    }

    let formatter: MultiOptionFormatter<'_, u64> = &|a| format!("{} pools", a.len());
    let ans = MultiSelect::new("Select a pool:", pool_ids)
        .with_formatter(formatter)
        .prompt();

    match ans {
        Ok(selection) => {
            println!("You selected: {:?}", selection);
            let args = App {
                command: Some(Act(ActArgs {
                    function: "swap".to_string(),
                    pool_id: selection[0].to_string(),
                    verbose: Some(false),
                    args: None,
                })),
            };

            invoke::main(&args).await?;
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    // Print an exit message
    let exit_message = format!("{}{}", "Done listing pools".bright_green(), " 🤗");
    println!("{}", exit_message);
    Ok(())
}
