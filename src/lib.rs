extern crate serde;
extern crate reqwest;

use serde::Deserialize;
use reqwest::{Response, Result};

mod endpoints;

const ORIGIN: &str = "https://data.elrond.com/";

#[derive(Deserialize, Debug)]
pub struct DataPoint<T> {
    #[allow(unused)]
    pub time: String,
    #[allow(unused)]
    pub value: T,
}

pub async fn fetch(endpoint: &str) -> Result<Response> {
    let url = format!("{}{}", ORIGIN, endpoint);
    let res = reqwest::get(&url).await?;
    println!("status: {:#?}", res.status());
    Ok(res)
}
 
// Market value
pub mod market_value {
 
    use super::Result;
    use super::DataPoint;
    use super::endpoints;
    use super::fetch;

    pub async fn price() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::PRICE).await?;
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn market_cap() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::MARKET_CAP).await?;
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn exchange_volume_24h() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::VOLUME_24H).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }


}

// Supply
pub mod supply {
    
    use super::Result;
    use super::DataPoint;
    use super::endpoints;
    use super::fetch;

    pub async fn circulating_supply() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::CIRCULATING_SUPPLY).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn floating_supply() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::FLOATING_SUPPLY).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn staked() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::STAKED).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

}

// User Adoption
pub mod user_adoption {
    
    use super::Result;
    use super::DataPoint;
    use super::endpoints;
    use super::fetch;

    pub async fn user_count() -> Result<Vec<DataPoint<u64>>> {
        let res = fetch(endpoints::USER_COUNT).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }
    
    pub async fn user_count_gt_0() -> Result<Vec<DataPoint<u64>>> {
        let res = fetch(endpoints::USER_COUNT_GT_0).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn user_count_gt_0_1() -> Result<Vec<DataPoint<u64>>> {
        let res = fetch(endpoints::USER_COUNT_GT_0_1).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }
    
    pub async fn user_count_gt_1() -> Result<Vec<DataPoint<u64>>> {
        let res = fetch(endpoints::USER_COUNT_GT_1).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn user_count_gt_10() -> Result<Vec<DataPoint<u64>>> {
        let res = fetch(endpoints::USER_COUNT_GT_10).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn user_count_gt_100() -> Result<Vec<DataPoint<u64>>> {
        let res = fetch(endpoints::USER_COUNT_GT_100).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }
    
    pub async fn user_count_gt_1000() -> Result<Vec<DataPoint<u64>>> {
        let res = fetch(endpoints::USER_COUNT_GT_1000).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

}

// Transactions
pub mod transactions {
    
    use super::Result;
    use super::DataPoint;
    use super::endpoints;
    use super::fetch;

    pub async fn total_transactions() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::TRANSACTIONS_TOTAL).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn transactions_24h() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::TRANSACTIONS_24H).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

}

// Staking Metrics
pub mod staking_metrics {
    
    use super::Result;
    use super::DataPoint;
    use super::endpoints;
    use super::fetch;

    pub async fn queued_value_total() -> Result<Vec<DataPoint<u64>>> {
        let res = fetch(endpoints::QUEUED_VALUE_TOTAL).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn queued_user_total() -> Result<Vec<DataPoint<u64>>> {
        let res = fetch(endpoints::QUEUED_USERS_TOTAL).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn queued_staked_total() -> Result<Vec<DataPoint<u64>>> {
        let res = fetch(endpoints::QUEUED_STAKED_TOTAL).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn queue_staking_total() -> Result<Vec<DataPoint<u64>>> {
        let res = fetch(endpoints::QUEUE_STAKING_TOTAL).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn queued_delegated_total() -> Result<Vec<DataPoint<u64>>> {
        let res = fetch(endpoints::QUEUE_DELEGATED_TOTAL).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn queue_delegating_total() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::QUEUE_DELEGATING_TOTAL).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }
}

// Exchanges
pub mod exchanges {

    use super::Result;
    use super::DataPoint;
    use super::endpoints;
    use super::fetch;

    pub async fn exchanges_total_balance() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(super::endpoints::EXCHANGES_TOTAL_BALANCE).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn bitmax_balance() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::BITMAX_BALANCE).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn binance_com_balance() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::BINANCE_COM_BALANCE).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn binance_us_balance() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::BINANCE_US_BALANCE).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }


    pub async fn bitfinex_balance() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::BITFINEX_BALANCE).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn bithumb_balance() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::BITHUMB_BALANCE).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn crypto_com_balance() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::CRYPTO_COM_BALANCE).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn kucoin_balance() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::KUCOIN_BALANCE).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn okex_balance() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::OKEX_BALANCE).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn inflow_24h() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::INFLOW_24).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn outflow_24h() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::OUTFLOW_24).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }


}

// Socials
pub mod socials {

    use super::Result;
    use super::DataPoint;
    use super::endpoints;
    use super::fetch;

    pub async fn mentions() -> Result<Vec<DataPoint<u64>>> {
        let res = fetch(endpoints::MENTIONS).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

    pub async fn impressions() -> Result<Vec<DataPoint<f64>>> {
        let res = fetch(endpoints::IMPRESSIONS).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }
}

pub mod dev_activity {
    use super::{fetch, endpoints, Result, DataPoint};

    pub async fn commits() -> Result<u32> {
        let res = fetch(endpoints::COMMITS).await?;
        let commits = res.json().await?;
        Ok(commits)
    }
    
    pub async fn stars() -> Result<u32> {
        let res = fetch(endpoints::STARS).await?;
        let stars = res.json().await?;
        Ok(stars)
    }

    pub async fn commits_24h() -> Result<Vec<DataPoint<u64>>> {
        let res = fetch(endpoints::COMMITS_24H).await?; 
        let data_points = res.json().await?;
        Ok(data_points)
    }

}
