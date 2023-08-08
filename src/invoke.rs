use super::{act, actions, info, list, App, Commands, Config};
use async_recursion::async_recursion;

/// Handles invoking commands from the cli or other modules.
#[async_recursion(?Send)]
pub async fn main(args: &App) -> Result<(), anyhow::Error> {
    let settings: Config = Config::new().unwrap();
    match &args.command {
        Some(Commands::List {}) => list::list_pools(&settings).await?,
        Some(Commands::Info { pool_id }) => info::main(&settings, pool_id).await?,
        Some(Commands::Act(args)) => act::main(&settings, args).await?,
        None => {
            println!("no command");
        }
    }

    Ok(())
}
