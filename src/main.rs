extern crate elrond_growth_rust;

use elrond_growth_rust::{market_value, EGResult};


#[tokio::main]
async fn main() -> EGResult<()> {
    let price_data = market_value::price().await?;
    println!("{:#?}", price_data.last().unwrap().value);
    Ok(())
}