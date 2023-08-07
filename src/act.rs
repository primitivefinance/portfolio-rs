use super::actions::{self, Actions};
use super::Config;

/// Handles actions
pub async fn main(cfg: &Config, action: Actions) -> Result<(), anyhow::Error> {
    match action {
        Actions::Swap { .. } => actions::swap::main(cfg).await?,
    }

    Ok(())
}
