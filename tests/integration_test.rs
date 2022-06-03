use elrond_growth_rust::{market_value, EGResult};

#[tokio::test]
async fn get_price() -> EGResult<()> {
    market_value::price().await?;
    Ok(())
}