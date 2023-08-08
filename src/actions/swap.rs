/// Handles swap actions
use crate::Config;
use anyhow::{self, Context};
use bindings::{i_portfolio::IPortfolio, i_portfolio_actions::SwapCall, shared_types::Order};
use colored::Colorize;
use ethers::{
    prelude::*,
    utils::{format_ether, parse_ether},
};
use std::sync::Arc;

#[derive(Clone)]
struct SwapArgs {
    sell_asset: bool,
    amount: f64,
    price: f64,
    slippage: f64,
}

impl Default for SwapArgs {
    fn default() -> Self {
        Self {
            sell_asset: false,
            amount: 0.0,
            price: 0.0,
            slippage: 0.0,
        }
    }
}

impl SwapArgs {
    fn new(sell_asset: bool, amount: f64, price: f64, slippage: f64) -> Self {
        Self {
            sell_asset,
            amount,
            price,
            slippage,
        }
    }

    fn from_cli(args: Vec<String>) -> Result<Self, anyhow::Error> {
        if args.len() < 2 {
            return Err(anyhow::anyhow!("Missing input argument"));
        }

        let mut swap_args = SwapArgs::default();

        let sell_asset = args[0].parse::<bool>().unwrap();
        let amount = args[1].parse::<f64>().unwrap();

        swap_args.sell_asset = sell_asset;
        swap_args.amount = amount;

        if args.len() > 2 {
            let price = args[2].parse::<f64>();
            let slippage = args[3].parse::<f64>();

            // Uses default
            if price.is_ok() {
                swap_args.price = price.unwrap();
            }

            // Uses default
            if slippage.is_ok() {
                swap_args.slippage = slippage.unwrap();
            }
        }

        Ok(swap_args)
    }

    async fn prepare(
        &self,
        contract: &IPortfolio<SignerMiddleware<Provider<Ws>, LocalWallet>>,
        pool_id: u64,
    ) -> Result<Order, anyhow::Error> {
        let preview_msg = format!(
            "\n{}",
            "Previewing swap... please be patient\n".yellow().bold()
        );
        println!("{}", preview_msg.on_black());

        let mut swap_call = parse_args(pool_id, self.clone())?;

        let current_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let signer_address = contract.client().address();

        let amount_out: U256 = contract
            .get_amount_out(
                pool_id,
                swap_call.sell_asset.into(),
                swap_call.input.into(),
                signer_address.into(),
            )
            .await
            .context(format!(
                "swap.rs: Failed to get amount out {:#?}",
                swap_call.clone()
            ))?;

        let spot_price: U256 = contract.get_spot_price(pool_id).await.context(format!(
            "swap.rs: Failed to get spot price {:#?}",
            swap_call.clone()
        ))?;

        // Overwrite the output amount.
        swap_call.output = amount_out.as_u128();

        let (success, _, _) = contract
            .simulate_swap(swap_call.clone(), current_timestamp.into(), signer_address)
            .await
            .context("swap.rs: Failed to simulate swap")?;

        let (bid, ask) = compute_bid_ask(spot_price, self.slippage)?;
        let mark_price =
            compute_mark_price(swap_call.sell_asset, swap_call.input.into(), amount_out)?;

        let print_prices_formatted_with_colors = |bid: U256, ask: U256, mark_price: U256| {
            let msg = format!(
                "\n{} {} {} {}
                {} {} {} {}
                {} {} {} {}",
                "\nDesired Price:".bold().blue(),
                self.price.to_string().bold().blue(),
                "\nDesired Slippage:".bold().blue(),
                self.slippage.to_string().bold().blue(),
                "\nBid:".bold().blue(),
                format_ether(bid).to_string().bold().blue(),
                "\nAsk:".bold().blue(),
                format_ether(ask).to_string().bold().blue(),
                "\nMark Price:".bold().blue(),
                format_ether(mark_price).to_string().bold().blue(),
                "\nSpot Price:".bold().blue(),
                format_ether(spot_price).to_string().bold().blue(),
            );
            println!("{}", msg.on_black());
        };

        print_prices_formatted_with_colors(bid, ask, mark_price);

        if !success {
            return Err(anyhow::anyhow!("Swap simulation failed"));
        } else {
            let success_msg = format!(
                "
                {} {} {} {} {} {} {}
                ",
                "Swap simulation successful".bold().green(),
                "🤑\n",
                "Result:\n".purple(),
                "Input:".bold().purple(),
                format_ether::<U256>(swap_call.input.into())
                    .to_string()
                    .purple(),
                "Output:".bold().purple(),
                format_ether::<U256>(swap_call.output.into())
                    .to_string()
                    .purple(),
            );
            println!("{}", success_msg.on_black());
        }

        Ok(swap_call)
    }
}

/// Executes the `swap` function on Portfolio.
pub async fn main(
    cfg: &Config,
    pool_id: u64,
    args: &Option<Vec<String>>,
) -> Result<(), anyhow::Error> {
    let client = Arc::new({
        let ws_provider = Provider::<Ws>::connect(&cfg.rpc_url).await?;

        let chain_id = ws_provider.get_chainid().await?;
        let wallet = std::env::var("PRIVATE_KEY")
            .context("swap.rs: PRIVATE_KEY env var not set")?
            .parse::<LocalWallet>()?
            .with_chain_id(chain_id.as_u64());

        SignerMiddleware::new(ws_provider, wallet)
    });

    let portfolio: IPortfolio<SignerMiddleware<Provider<Ws>, _>> =
        IPortfolio::new(cfg.portfolio_address.parse::<Address>()?, client);
    let version = portfolio.version().call().await?;
    let version_msg = format!(
        "
        {} {} {}
        ",
        "Portfolio version:".yellow(),
        version.to_string().bold().yellow(),
        "\n"
    );
    println!("{}", version_msg.on_black());

    if let Some(args) = args {
        let swap_args = SwapArgs::from_cli(args.clone())?;
        do_swap(&portfolio, pool_id, swap_args).await?;
    } else {
        println!("Missing input arguments, try passing --args <sell_asset> <amount> <price> <slippage> to the swap action.");
    }
    Ok(())
}

/// Converts an optional Vec<String> into a the correct swap arguments.
fn parse_args(pool_id: u64, swap_args: SwapArgs) -> Result<Order, anyhow::Error> {
    let mut order = Order::default();
    order.pool_id = pool_id;

    order.sell_asset = swap_args.sell_asset;
    order.input = parse_ether(swap_args.amount)?.as_u128();

    if order.pool_id == 0 {
        return Err(anyhow::anyhow!("Invalid pool id"));
    }

    Ok(order)
}

/// Gracefully executes a swap transaction on Portfolio and propagates any errors.
async fn do_swap(
    portfolio: &IPortfolio<SignerMiddleware<Provider<Ws>, LocalWallet>>,
    pool_id: u64,
    args: SwapArgs,
) -> Result<(), anyhow::Error> {
    let mut swap_args = args.prepare(portfolio, pool_id).await?;
    let mut result = portfolio
        .swap(swap_args.clone())
        .await
        .context("swap.rs: Failed to execute swap")?;

    let success_msg = format!(
        "{} {} {} {:#?}",
        "Swap successful".bold().green(),
        "🤑\n",
        "Result:\n".purple(),
        result
    );
    println!("{}", success_msg.on_black());
    Ok(())
}

/// Computes the price for buying or selling an asset at a price and slippage.
fn compute_bid_ask(price: U256, slippage: f64) -> Result<(U256, U256), anyhow::Error> {
    let bid = price
        .checked_mul(parse_ether(1.0 - slippage)?)
        .ok_or(anyhow::anyhow!("Overflow"))?
        .checked_div(parse_ether(1.0)?)
        .ok_or(anyhow::anyhow!("Overflow"))?;
    let ask = price
        .checked_mul(parse_ether(1.0 + slippage)?)
        .ok_or(anyhow::anyhow!("Overflow"))?
        .checked_div(parse_ether(1.0)?)
        .ok_or(anyhow::anyhow!("Overflow"))?;
    Ok((bid, ask))
}

/// Assuming a simulated trade was successful with input and output amounts, computes the price from the trade.
fn compute_mark_price(
    sell_asset: bool,
    amount_in: U256,
    amount_out: U256,
) -> Result<U256, anyhow::Error> {
    if sell_asset {
        Ok(amount_in
            .checked_mul(parse_ether(1.0)?)
            .ok_or(anyhow::anyhow!("Overflow"))?
            .checked_div(amount_out)
            .ok_or(anyhow::anyhow!("compute_mark_price.sell_asset.Overflow"))?)
    } else {
        Ok(amount_out
            .checked_mul(parse_ether(1.0)?)
            .ok_or(anyhow::anyhow!("Overflow"))?
            .checked_div(amount_in)
            .ok_or(anyhow::anyhow!("compute_mark_price.Overflow"))?)
    }
}
