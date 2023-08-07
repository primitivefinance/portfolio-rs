use alloy_primitives::{Address, U256};
use alloy_sol_types::{sol, SolCall};
use hex_literal::hex;

use config::{Config, ConfigError, Environment};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Swap {
    pool_id: String,
    min_price: f64,
    max_price: f64,
}

#[derive(Debug, Deserialize)]
struct Settings {
    swap: Swap,
}

/// Enters the portfolio-rs cli application.
fn main() {
    println!("{}", WELCOME);

    let settings = Settings::new().unwrap();

    println!("settings: {:?}", settings);
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name("portfolio"))
            .add_source(Environment::with_prefix("PORTFOLIO"))
            .build()?;

        s.try_deserialize()
    }
}

static WELCOME: &str = "
░█▀█░█▀█░█▀▄░▀█▀░█▀▀░█▀█░█░░░▀█▀░█▀█░
░█▀▀░█░█░█▀▄░░█░░█▀▀░█░█░█░░░░█░░█░█░
░▀░░░▀▀▀░▀░▀░░▀░░▀░░░▀▀▀░▀▀▀░▀▀▀░▀▀▀░
";

// `cargo test -- --nocapture` to see the output.
#[cfg(test)]
mod tests {
    use super::*;

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
