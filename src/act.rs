use ethers::abi::{Function, Token};

use super::actions::{self, Actions};
use super::ActArgs;
use super::Config;

/// Handles the "Act" command
pub async fn main(cfg: &Config, args: &ActArgs) -> Result<(), anyhow::Error> {
    let action = match &args.function {
        action if action == "swap" => {
            let id: u64 = args.pool_id.parse::<u64>()?;
            actions::swap::main(cfg, id, &args.args).await?
        }
        _ => unimplemented!("not implemented yet"),
    };

    Ok(())
}
