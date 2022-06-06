#![allow(dead_code)]
pub const PRICE: &str = "marketcomplete/quoteshistorical/egld/price";
pub const MARKET_CAP: &str = "marketcomplete/quoteshistorical/egld/market_cap";
pub const VOLUME_24H: &str = "marketcomplete/quoteshistorical/egld/volume_24h";

// Supply
pub const CIRCULATING_SUPPLY: &str = "latestseries/economics/economics/circulating_supply";
pub const STAKED: &str = "latestseries/economics/economics/staked";
pub const FLOATING_SUPPLY: &str = "latestseries/economics/economics/floating_supply";

// Transactions
pub const TRANSACTIONS_24H: &str = "complete/transactionshistorical/transactions/count_24h";
pub const TRANSACTIONS_TOTAL: &str = "latestcomplete/transactionshistorical/transactions/count";

// Staking metrics
pub const QUEUED_VALUE_TOTAL: &str = "latestseries/staking/total/waiting_list_value";
pub const QUEUED_USERS_TOTAL: &str = "latestseries/staking/total/waiting_list";
pub const QUEUED_STAKED_TOTAL: &str = "latestseries/staking/staking/waiting_list_value";    
pub const QUEUE_STAKING_TOTAL: &str = "latestseries/staking/staking/waiting_list";
pub const QUEUE_DELEGATED_TOTAL: &str = "latestseries/staking/delegation/waiting_list_value";
pub const QUEUE_DELEGATING_TOTAL: &str = "latestseries/staking/delegation/waiting_list";

pub const TOTAL_STAKED: &str = "latestseries/economics/economics/staked";
pub const TOTAL_USERS_STAKING: &str = "series/stakinghistorical/total/users";
pub const ACTIVE_STAKED:&str = "latestseries/stakinghistorical/staking/value";
pub const ACTIVE_USERS_STAKING: &str = "latestseries/stakinghistorical/staking/user";
pub const LEGACY_DELEGATED_STAKED: &str = "stakinghistorical/legacydelegation/value";
pub const LEGACY_DELEGATED_USERS_STAKING: &str = "series/stakinghistorical/legacydelegation/users";
pub const DELEGATION_ACTIVE_STAKED: &str = "latestseries/stakinghistorical/delegation/value";
pub const DELEGATION_ACTIVE_STAKING: &str = "series/stakinghistorical/delegation/users";
pub const ACTIVE_DELEGATED: &str = "latestseries/staking/delegation/active_value";
pub const ACTIVE_USER_DELEGATING: &str = "latestseries/staking/delegation/active";


// Exchanges
pub const EXCHANGES_TOTAL_BALANCE: &str = "latestseries/exchanges/total/balance";
pub const BITMAX_BALANCE: &str = "latestseries/exchanges/bitmax/balance";
pub const BINANCE_COM_BALANCE: &str = "latestseries/exchanges/binance_com/balance";
pub const BINANCE_US_BALANCE: &str = "latestseries/exchanges/binance_us/balance";
pub const BITFINEX_BALANCE: &str = "latestseries/exchanges/bitfinex/balance";
pub const BITHUMB_BALANCE: &str = "latestseries/exchanges/bithumb/balance";
pub const CRYPTO_COM_BALANCE: &str = "latestseries/exchanges/crypto_com/balance";
pub const KUCOIN_BALANCE: &str = "latestseries/exchanges/kucoin/balance";
pub const OKEX_BALANCE: &str = "latestseries/exchanges/okex/balance";
pub const INFLOW_24: &str = "latestseries/exchanges/total/inflow_24h";
pub const OUTFLOW_24: &str = "latestseries/exchanges/total/outflow_24h";

// Dev Activity
pub const COMMITS: &str = "latest/githubactivity/total/commits";
pub const COMMITS_24H: &str = "latestseries/githubactivity/total/commits_24h";
pub const STARS: &str = "latest/githubactivity/total/stars";

// User Adaoption
pub const USER_COUNT: &str = "latestcomplete/accountshistorical/accounts/count";
pub const USER_COUNT_GT_0: &str = "latestcomplete/accountshistorical/totalbalancewithstake/count_gt_0";
pub const USER_COUNT_GT_0_1: &str = "latestcomplete/accountshistorical/totalbalancewithstake/count_gt_0_1";
pub const USER_COUNT_GT_1: &str = "latestcomplete/accountshistorical/totalbalancewithstake/count_gt_1";
pub const USER_COUNT_GT_10: &str = "latestcomplete/accountshistorical/totalbalancewithstake/count_gt_10";
pub const USER_COUNT_GT_100: &str = "latestcomplete/accountshistorical/totalbalancewithstake/count_gt_100";
pub const USER_COUNT_GT_1000: &str = "latestcomplete/accountshistorical/totalbalancewithstake/count_gt_1000";

// Socials
pub const MENTIONS: &str = "series/twitter/twitter/mentions";
pub const IMPRESSIONS: &str = "series/google/total/impressions";