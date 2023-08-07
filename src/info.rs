use super::Config;
use colored::Colorize;

pub async fn main(cfg: &Config, pool_id: &str) -> Result<(), anyhow::Error> {
    let _ = cfg;
    println!(
        "{} {} {} {}",
        "Getting info for pool".blue(),
        pool_id.bold().magenta(),
        "please be patient...".blue(),
        " 🤗"
    );
    Ok(())
}
