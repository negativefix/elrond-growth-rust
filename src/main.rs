use elrond_growth_rust::market_value;
use reqwest::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let price_data = market_value::price().await?;
    println!("{:#?}", price_data.last().unwrap().value);
    Ok(())
}