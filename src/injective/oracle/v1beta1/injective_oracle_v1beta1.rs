#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleInfo {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(enumeration="OracleType", tag="2")]
    pub oracle_type: i32,
    #[prost(uint32, tag="3")]
    pub scale_factor: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainlinkPriceState {
    #[prost(string, tag="1")]
    pub feed_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub answer: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub timestamp: u64,
    #[prost(message, optional, tag="4")]
    pub price_state: ::core::option::Option<PriceState>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BandPriceState {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub rate: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub resolve_time: u64,
    #[prost(uint64, tag="4")]
    pub request_id: u64,
    #[prost(message, optional, tag="5")]
    pub price_state: ::core::option::Option<PriceState>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceFeedState {
    #[prost(string, tag="1")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub quote: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub price_state: ::core::option::Option<PriceState>,
    #[prost(string, repeated, tag="4")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderInfo {
    #[prost(string, tag="1")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderState {
    #[prost(message, optional, tag="1")]
    pub provider_info: ::core::option::Option<ProviderInfo>,
    #[prost(message, repeated, tag="2")]
    pub provider_price_states: ::prost::alloc::vec::Vec<ProviderPriceState>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderPriceState {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub state: ::core::option::Option<PriceState>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceFeedInfo {
    #[prost(string, tag="1")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub quote: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceFeedPrice {
    #[prost(string, tag="1")]
    pub price: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoinbasePriceState {
    /// kind should always be "prices"
    #[prost(string, tag="1")]
    pub kind: ::prost::alloc::string::String,
    /// timestamp of the when the price was signed by coinbase
    #[prost(uint64, tag="2")]
    pub timestamp: u64,
    /// the symbol of the price, e.g. BTC
    #[prost(string, tag="3")]
    pub key: ::prost::alloc::string::String,
    /// the value of the price scaled by 1e6
    #[prost(uint64, tag="4")]
    pub value: u64,
    /// the price state
    #[prost(message, optional, tag="5")]
    pub price_state: ::core::option::Option<PriceState>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceState {
    #[prost(string, tag="1")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub cumulative_price: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub timestamp: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BandOracleRequest {
    /// Unique Identifier for band ibc oracle request
    #[prost(uint64, tag="1")]
    pub request_id: u64,
    /// OracleScriptID is the unique identifier of the oracle script to be executed.
    #[prost(int64, tag="2")]
    pub oracle_script_id: i64,
    /// Symbols is the list of symbols to prepare in the calldata
    #[prost(string, repeated, tag="3")]
    pub symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// AskCount is the number of validators that are requested to respond to this
    /// oracle request. Higher value means more security, at a higher gas cost.
    #[prost(uint64, tag="4")]
    pub ask_count: u64,
    /// MinCount is the minimum number of validators necessary for the request to
    /// proceed to the execution phase. Higher value means more security, at the
    /// cost of liveness.
    #[prost(uint64, tag="5")]
    pub min_count: u64,
    /// FeeLimit is the maximum tokens that will be paid to all data source providers.
    #[prost(message, repeated, tag="6")]
    pub fee_limit: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// PrepareGas is amount of gas to pay to prepare raw requests
    #[prost(uint64, tag="7")]
    pub prepare_gas: u64,
    /// ExecuteGas is amount of gas to reserve for executing
    #[prost(uint64, tag="8")]
    pub execute_gas: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BandIbcParams {
    /// true if Band IBC should be enabled
    #[prost(bool, tag="1")]
    pub band_ibc_enabled: bool,
    /// block request interval to send Band IBC prices
    #[prost(int64, tag="2")]
    pub ibc_request_interval: i64,
    /// band IBC source channel
    #[prost(string, tag="3")]
    pub ibc_source_channel: ::prost::alloc::string::String,
    /// band IBC version
    #[prost(string, tag="4")]
    pub ibc_version: ::prost::alloc::string::String,
    /// band IBC portID
    #[prost(string, tag="5")]
    pub ibc_port_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymbolPriceTimestamp {
    #[prost(enumeration="OracleType", tag="1")]
    pub oracle: i32,
    #[prost(string, tag="2")]
    pub symbol_id: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub timestamp: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPriceTimestamps {
    #[prost(message, repeated, tag="1")]
    pub last_price_timestamps: ::prost::alloc::vec::Vec<SymbolPriceTimestamp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceRecords {
    #[prost(enumeration="OracleType", tag="1")]
    pub oracle: i32,
    #[prost(string, tag="2")]
    pub symbol_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub latest_price_records: ::prost::alloc::vec::Vec<PriceRecord>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceRecord {
    #[prost(int64, tag="1")]
    pub timestamp: i64,
    #[prost(string, tag="2")]
    pub price: ::prost::alloc::string::String,
}
/// MetadataStatistics refers to the metadata summary statistics of the historical sample considered
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataStatistics {
    /// GroupCount refers to the number of groups used. Equals RecordsSampleSize if no grouping is used
    #[prost(uint32, tag="1")]
    pub group_count: u32,
    /// RecordsSampleSize refers to the total number of records used.
    #[prost(uint32, tag="2")]
    pub records_sample_size: u32,
    /// Mean refers to the arithmetic mean
    /// For trades, the mean is the VWAP computed over the grouped trade records ∑ (price * quantity) / ∑ quantity
    /// For oracle prices, the mean is computed over the price records ∑ (price) / prices_count
    #[prost(string, tag="3")]
    pub mean: ::prost::alloc::string::String,
    /// TWAP refers to the time-weighted average price which equals ∑ (price_i * ∆t_i) / ∑ ∆t_i where ∆t_i = t_i - t_{i-1}
    #[prost(string, tag="4")]
    pub twap: ::prost::alloc::string::String,
    /// FirstTimestamp is the timestamp of the oldest record considered
    #[prost(int64, tag="5")]
    pub first_timestamp: i64,
    /// LastTimestamp is the timestamp of the youngest record considered
    #[prost(int64, tag="6")]
    pub last_timestamp: i64,
    /// MinPrice refers to the smallest individual raw price considered
    #[prost(string, tag="7")]
    pub min_price: ::prost::alloc::string::String,
    /// MaxPrice refers to the largest individual raw price considered
    #[prost(string, tag="8")]
    pub max_price: ::prost::alloc::string::String,
    /// MedianPrice refers to the median individual raw price considered
    #[prost(string, tag="9")]
    pub median_price: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OracleType {
    Unspecified = 0,
    Band = 1,
    PriceFeed = 2,
    Coinbase = 3,
    Chainlink = 4,
    Razor = 5,
    Dia = 6,
    Api3 = 7,
    Uma = 8,
    Pyth = 9,
    BandIbc = 10,
    Provider = 11,
}
impl OracleType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OracleType::Unspecified => "Unspecified",
            OracleType::Band => "Band",
            OracleType::PriceFeed => "PriceFeed",
            OracleType::Coinbase => "Coinbase",
            OracleType::Chainlink => "Chainlink",
            OracleType::Razor => "Razor",
            OracleType::Dia => "Dia",
            OracleType::Api3 => "API3",
            OracleType::Uma => "Uma",
            OracleType::Pyth => "Pyth",
            OracleType::BandIbc => "BandIBC",
            OracleType::Provider => "Provider",
        }
    }
}