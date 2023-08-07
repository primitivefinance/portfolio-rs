use alloy_primitives::Address;
use clap::{Args, Parser, Subcommand};

use dotenv::dotenv;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

mod act;
mod actions;
mod info;
mod list;
mod utils;

/// # Portfolio rs
/// Rust cli for fetching and interacting with the Portfolio protocol on supported networks.
///
/// ## Commands
/// - `list` - Lists all the pools, including pool id, tokens, and estimated TVL if available.
/// - `info` - Prints a pool's state and configuration, if any.
/// - `action` - Performs an action on a pool, such as swap, add liquidity, remove liquidity, etc. [Required] Settings in portfolio.toml.
#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    println!("{}", WELCOME);

    dotenv().ok();

    let settings: Config = Config::new().unwrap();
    let args = App::parse();

    match &args.command {
        Some(Commands::List {}) => list::list_pools(&settings).await?,
        Some(Commands::Info { pool_id }) => info::main(&settings, pool_id).await?,
        Some(Commands::Act(args)) => {
            let action = match &args.action {
                action if action == "swap" => actions::Actions::Swap,
                _ => unimplemented!("not implemented yet"),
            };

            act::main(&settings, action).await?
        }
        None => {
            println!("no command");
        }
    }

    Ok(())
}

static WELCOME: &str = "
░█▀█░█▀█░█▀▄░▀█▀░█▀▀░█▀█░█░░░▀█▀░█▀█░
░█▀▀░█░█░█▀▄░░█░░█▀▀░█░█░█░░░░█░░█░█░
░▀░░░▀▀▀░▀░▀░░▀░░▀░░░▀▀▀░▀▀▀░▀▀▀░▀▀▀░
";

// =================== CONFIG ===================

/// Configuration for doing swap actions on portfolio.
#[derive(Debug, Deserialize)]
#[allow(unused)]
#[serde(default)]
struct Swap {
    pool_id: String,
    min_price: f64,
    max_price: f64,
}

impl Default for Swap {
    fn default() -> Self {
        Self {
            pool_id: "".to_string(),
            min_price: 0.0,
            max_price: 0.0,
        }
    }
}

/// Configuration of portfolio-rs
#[derive(Debug, Deserialize)]
#[allow(unused)]
#[serde(default)]
pub struct Config {
    name: String,
    rpc_url: String,
    factory_address: String,
    portfolio_address: String,
    swap: Swap,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: "Default profile".to_string(),
            rpc_url: "https://mainnet.infura.io/v3/".to_string(),
            factory_address: Address::ZERO.to_string(),
            portfolio_address: Address::ZERO.to_string(),
            swap: Swap::default(),
        }
    }
}

impl Config {
    pub fn new() -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Toml::file("portfolio.toml").nested())
            .merge(Env::prefixed("PORTFOLIO_"))
            .join(Serialized::defaults(App::parse()))
            .extract()
    }
}

// =================== CLI ===================

/// # Commands
/// Main program.
///
/// ### Usage
/// $ port <command> <args>
#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(name = "portfolio-rs", version = "0.1.0", about = "Portfolio-rs cli.")]
struct App {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// ## Subcommands.
/// Subcommands for the main program.
///
/// ### Usage
/// $ port list
/// $ port info -p <pool_id>
/// $ port act -p <pool_id> -a <action>
#[derive(Subcommand, Debug, Serialize, Deserialize)]
enum Commands {
    /// Lists all the pools.
    List {},
    /// Prints a pool's state and configuration.
    Info {
        #[arg(short, long)]
        pool_id: String,
    },
    /// Performs an action on a pool, such as swap, add liquidity, remove liquidity, etc.
    Act(ActArgs),
}

/// # Act
/// Performs an action on a pool, such as swap, add liquidity, remove liquidity, etc.
///
/// ## ActArgs
/// Pass the pool id and specific action to execute in the `act` subcommand.
///
/// ### Usage
/// $ port act -p <pool_id> -a <action>
#[derive(Debug, Args, Serialize, Deserialize)]
struct ActArgs {
    /// Function to call on Portfolio.
    #[arg(short, long)]
    action: String,
    /// Pool id to execute the action on.
    #[arg(short, long)]
    pool_id: String,
    /// Print all available logs while action is pending.
    verbose: Option<bool>,
}

// =================== Tests ===================

// `cargo test -- --nocapture` to see the output.
#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::U256;
    use alloy_sol_types::{sol, SolCall};
    use hex_literal::hex;

    #[test]
    fn it_works() {
        sol! {
            #[derive(Debug, PartialEq)]
            interface IERC20 {
                function transfer(address to, uint256 amount) external returns (bool);
                function approve(address spender, uint256 amount) external returns (bool);
            }
        }

        // random mainnet ERC20 transfer
        // https://etherscan.io/tx/0x947332ff624b5092fb92e8f02cdbb8a50314e861a4b39c29a286b3b75432165e
        let data = hex!(
            "a9059cbb"
            "0000000000000000000000008bc47be1e3abbaba182069c89d08a61fa6c2b292"
            "0000000000000000000000000000000000000000000000000000000253c51700"
        );
        let expected = IERC20::transferCall {
            to: Address::from(hex!("8bc47be1e3abbaba182069c89d08a61fa6c2b292")),
            amount: U256::from(9995360000_u64),
        };

        assert_eq!(data[..4], IERC20::transferCall::SELECTOR);
        let decoded = IERC20::IERC20Calls::decode(&data, true).unwrap();
        assert_eq!(decoded, IERC20::IERC20Calls::transfer(expected));
        assert_eq!(decoded.encode(), data);

        println!("decoded transfer call result: {:?}", decoded);
    }
}
