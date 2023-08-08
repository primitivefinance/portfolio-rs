use colored::Colorize;

use super::{ActArgs, App, Commands::Act, Config};
use crate::invoke;
use anyhow;
use clap::{Arg, Command};
use ethers::prelude::*;

use inquire::{formatter::OptionFormatter, Select};
use std::sync::Arc;

use bindings::i_portfolio::{CreatePoolFilter, IPortfolio};

/// Lists the pools of a Portfolio contract.
pub async fn list_pools(cfg: &Config) -> Result<(), anyhow::Error> {
    let ws_provider = Provider::<Ws>::connect(&cfg.rpc_url).await?;
    let ws_client = Arc::new(ws_provider);
    let connected_msg = format!(
        "{} {} {} {} {}",
        "Connected".yellow(),
        "\n   - RPC:".yellow(),
        &cfg.rpc_url.bold().yellow(),
        "\n   - Portfolio:".yellow(),
        &cfg.portfolio_address.bold().yellow()
    );
    println!("{}", connected_msg.on_black());

    let start_block = 3982259;
    let address = (&cfg.portfolio_address).parse::<Address>()?;
    let contract = IPortfolio::new(address, ws_client);

    let events: Vec<CreatePoolFilter> = contract
        .create_pool_filter()
        .from_block(start_block)
        .query()
        .await?;

    let listing_pools_msg = format!("{}{}", "Listing pools... please be patient".yellow(), " 🤗");
    println!("{}", listing_pools_msg.on_black());

    let mut pool_ids = Vec::<u64>::new();
    for (i, event) in events.into_iter().enumerate() {
        if i == 0 {
            let found_msg = format!(
                "{} {} {}",
                "Found".green(),
                (i + 1).to_string().green().bold(),
                "pools".green()
            );
            println!("{}", found_msg.on_black());
        }

        pool_ids.push(event.pool_id);

        let pool_list_msg = format!(
            "   - {}{} {} {}",
            "#".purple(),
            i.to_string().bold().purple(),
            "- id:".purple(),
            event.pool_id.to_string().bold().purple()
        );
        println!("{}", pool_list_msg.on_black());
    }

    let formatter: OptionFormatter<'_, u64> = &|a| format!("{}", a);
    let ans = Select::new("Select a pool:", pool_ids)
        .with_formatter(formatter)
        .prompt();

    match ans {
        Ok(selected_pool_id) => {
            let selected_msg = format!(
                "{} {}",
                "You selected:".blue(),
                selected_pool_id.to_string().blue().bold()
            );
            println!("{}", selected_msg.on_black());

            let commands_with_pool = vec!["info".to_string(), "swap".to_string()];
            let formatter_with_pool: OptionFormatter<'_, String> = &|a| format!("{} commands", a);
            let answer_with_pool = Select::new("Select a command:", commands_with_pool)
                .with_formatter(formatter_with_pool)
                .prompt();

            match answer_with_pool {
                Ok(selected_command) => {
                    let selected_cmd_msg = format!(
                        "{} {}",
                        "You selected:".blue(),
                        selected_command.to_string().blue().bold()
                    );
                    println!("{}", selected_cmd_msg.on_black());

                    match selected_command.as_str() {
                        "info" => {
                            let args = App {
                                command: Some(super::Commands::Info {
                                    pool_id: selected_pool_id.to_string(),
                                }),
                            };
                            invoke::main(&args).await?;
                        }
                        "swap" => {
                            let sell_asset =
                                inquire::CustomType::<bool>::new("Do you want to sell asset?")
                                    .with_error_message(
                                        "Please enter a valid boolean value (true or false)",
                                    )
                                    .with_validator(|input: &bool| {
                                        if *input == true || *input == false {
                                            Ok(inquire::validator::Validation::Valid)
                                        } else {
                                            Ok(inquire::validator::Validation::Invalid(
                                        "Please enter a valid boolean value (true or false)".into(),
                                    ))
                                        }
                                    })
                                    .prompt()?;

                            let amount =
                                inquire::CustomType::<f64>::new("How much do you want to swap?")
                                    .with_formatter(&|i| format!("${i:.4}"))
                                    .with_error_message(
                                        "Please enter a valid number with up to 4 decimals.",
                                    )
                                    .prompt()?;

                            let args = App {
                                command: Some(super::Commands::Act(super::ActArgs {
                                    pool_id: selected_pool_id.to_string(),
                                    args: Some(vec![sell_asset.to_string(), amount.to_string()]),
                                    function: "swap".to_string(),
                                    verbose: None,
                                })),
                            };
                            invoke::main(&args).await?;
                        }
                        _ => {
                            println!("Error: Invalid command");
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    // Print an exit message
    let exit_message = format!("{}{}", "Done listing pools!".green(), " 🤗");
    println!("{}", exit_message.on_black());
    Ok(())
}
