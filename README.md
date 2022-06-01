# elrond-growth-rust 

Rust wrapper for Elrond Growth Api - https://growth.elrond.com


## Usage

```
use elrond_growth_rust::{market_value, EGResult};

#[tokio::main]
async fn main() -> EGResult<()> {
    let price_data = market_value::price().await?;
    println!("{:#?}", price_data.last().unwrap().value);
    Ok(())
}```