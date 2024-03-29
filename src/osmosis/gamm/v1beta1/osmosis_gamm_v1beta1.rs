/// ===================== MsgJoinPool
/// This is really MsgJoinPoolNoSwap
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinPool {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub share_out_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub token_in_maxs: ::prost::alloc::vec::Vec<
        super::super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinPoolResponse {
    #[prost(string, tag = "1")]
    pub share_out_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub token_in: ::prost::alloc::vec::Vec<
        super::super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// ===================== MsgExitPool
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitPool {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub share_in_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub token_out_mins: ::prost::alloc::vec::Vec<
        super::super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitPoolResponse {
    #[prost(message, repeated, tag = "1")]
    pub token_out: ::prost::alloc::vec::Vec<
        super::super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// ===================== MsgSwapExactAmountIn
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapAmountInRoute {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_out_denom: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountIn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
    #[prost(message, optional, tag = "3")]
    pub token_in: ::core::option::Option<
        super::super::super::super::cosmos::base::v1beta1::Coin,
    >,
    #[prost(string, tag = "4")]
    pub token_out_min_amount: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgSwapExactAmountOut
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapAmountOutRoute {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_in_denom: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountOut {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
    #[prost(string, tag = "3")]
    pub token_in_max_amount: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub token_out: ::core::option::Option<
        super::super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountOutResponse {
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// ===================== MsgJoinSwapExternAmountIn
/// TODO: Rename to MsgJoinSwapExactAmountIn
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinSwapExternAmountIn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(message, optional, tag = "3")]
    pub token_in: ::core::option::Option<
        super::super::super::super::cosmos::base::v1beta1::Coin,
    >,
    /// repeated cosmos.base.v1beta1.Coin tokensIn = 5 [
    ///    (gogoproto.moretags) = "yaml:\"tokens_in\"",
    ///    (gogoproto.nullable) = false
    /// ];
    #[prost(string, tag = "4")]
    pub share_out_min_amount: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinSwapExternAmountInResponse {
    #[prost(string, tag = "1")]
    pub share_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgJoinSwapShareAmountOut
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinSwapShareAmountOut {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub token_in_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub share_out_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub token_in_max_amount: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinSwapShareAmountOutResponse {
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// ===================== MsgExitSwapShareAmountIn
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitSwapShareAmountIn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub token_out_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub share_in_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub token_out_min_amount: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitSwapShareAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgExitSwapExternAmountOut
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitSwapExternAmountOut {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(message, optional, tag = "3")]
    pub token_out: ::core::option::Option<
        super::super::super::super::cosmos::base::v1beta1::Coin,
    >,
    #[prost(string, tag = "4")]
    pub share_in_max_amount: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitSwapExternAmountOutResponse {
    #[prost(string, tag = "1")]
    pub share_in_amount: ::prost::alloc::string::String,
}
/// =============================== Pool
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolResponse {
    #[prost(message, optional, tag = "1")]
    pub pool: ::core::option::Option<super::super::super::super::google::protobuf::Any>,
}
/// =============================== Pools
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolsResponse {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<super::super::super::super::google::protobuf::Any>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// =============================== NumPools
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNumPoolsRequest {}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNumPoolsResponse {
    #[prost(uint64, tag = "1")]
    pub num_pools: u64,
}
/// =============================== PoolType
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolTypeRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolTypeResponse {
    #[prost(string, tag = "1")]
    pub pool_type: ::prost::alloc::string::String,
}
/// =============================== CalcJoinPoolShares
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCalcJoinPoolSharesRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(message, repeated, tag = "2")]
    pub tokens_in: ::prost::alloc::vec::Vec<
        super::super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCalcJoinPoolSharesResponse {
    #[prost(string, tag = "1")]
    pub share_out_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub tokens_out: ::prost::alloc::vec::Vec<
        super::super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// =============================== CalcExitPoolCoinsFromShares
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCalcExitPoolCoinsFromSharesRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub share_in_amount: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCalcExitPoolCoinsFromSharesResponse {
    #[prost(message, repeated, tag = "1")]
    pub tokens_out: ::prost::alloc::vec::Vec<
        super::super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// =============================== PoolParams
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolParamsRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<super::super::super::super::google::protobuf::Any>,
}
/// =============================== PoolLiquidity
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalPoolLiquidityRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalPoolLiquidityResponse {
    #[prost(message, repeated, tag = "1")]
    pub liquidity: ::prost::alloc::vec::Vec<
        super::super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// =============================== TotalShares
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalSharesRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalSharesResponse {
    #[prost(message, optional, tag = "1")]
    pub total_shares: ::core::option::Option<
        super::super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// QuerySpotPriceRequest defines the gRPC request structure for a SpotPrice
/// query.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotPriceRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub base_asset_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_asset_denom: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolsWithFilterRequest {
    #[prost(message, repeated, tag = "1")]
    pub min_liquidity: ::prost::alloc::vec::Vec<
        super::super::super::super::cosmos::base::v1beta1::Coin,
    >,
    #[prost(string, tag = "2")]
    pub pool_type: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolsWithFilterResponse {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<super::super::super::super::google::protobuf::Any>,
}
/// QuerySpotPriceResponse defines the gRPC response structure for a SpotPrice
/// query.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotPriceResponse {
    /// String of the Dec. Ex) 10.203uatom
    #[prost(string, tag = "1")]
    pub spot_price: ::prost::alloc::string::String,
}
/// =============================== EstimateSwapExactAmountIn
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySwapExactAmountInRequest {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySwapExactAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// =============================== EstimateSwapExactAmountOut
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySwapExactAmountOutRequest {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(message, repeated, tag = "3")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
    #[prost(string, tag = "4")]
    pub token_out: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySwapExactAmountOutResponse {
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalLiquidityRequest {}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalLiquidityResponse {
    #[prost(message, repeated, tag = "1")]
    pub liquidity: ::prost::alloc::vec::Vec<
        super::super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// Parameters for changing the weights in a balancer pool smoothly from
/// a start weight and end weight over a period of time.
/// Currently, the only smooth change supported is linear changing between
/// the two weights, but more types may be added in the future.
/// When these parameters are set, the weight w(t) for pool time `t` is the
/// following:
///    t <= start_time: w(t) = initial_pool_weights
///    start_time < t <= start_time + duration:
///      w(t) = initial_pool_weights + (t - start_time) *
///        (target_pool_weights - initial_pool_weights) / (duration)
///    t > start_time + duration: w(t) = target_pool_weights
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmoothWeightChangeParams {
    /// The start time for beginning the weight change.
    /// If a parameter change / pool instantiation leaves this blank,
    /// it should be generated by the state_machine as the current time.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<
        super::super::super::super::google::protobuf::Timestamp,
    >,
    /// Duration for the weights to change over
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// The initial pool weights. These are copied from the pool's settings
    /// at the time of weight change instantiation.
    /// The amount PoolAsset.token.amount field is ignored if present,
    /// future type refactorings should just have a type with the denom & weight
    /// here.
    #[prost(message, repeated, tag = "3")]
    pub initial_pool_weights: ::prost::alloc::vec::Vec<PoolAsset>,
    /// The target pool weights. The pool weights will change linearly with respect
    /// to time between start_time, and start_time + duration. The amount
    /// PoolAsset.token.amount field is ignored if present, future type
    /// refactorings should just have a type with the denom & weight here.
    ///
    /// Intermediate variable for the 'slope' of pool weights. This is equal to
    /// (target_pool_weights - initial_pool_weights) / (duration)
    /// TODO: Work out precision, and decide if this is good to add
    /// repeated PoolAsset poolWeightSlope = 5 [
    ///   (gogoproto.moretags) = "yaml:\"pool_weight_slope\"",
    ///   (gogoproto.nullable) = false
    /// ];
    #[prost(message, repeated, tag = "4")]
    pub target_pool_weights: ::prost::alloc::vec::Vec<PoolAsset>,
}
/// PoolParams defined the parameters that will be managed by the pool
/// governance in the future. This params are not managed by the chain
/// governance. Instead they will be managed by the token holders of the pool.
/// The pool's token holders are specified in future_pool_governor.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolParams {
    #[prost(string, tag = "1")]
    pub swap_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub exit_fee: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub smooth_weight_change_params: ::core::option::Option<SmoothWeightChangeParams>,
}
/// Pool asset is an internal struct that combines the amount of the
/// token in the pool, and its balancer weight.
/// This is an awkward packaging of data,
/// and should be revisited in a future state migration.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolAsset {
    /// Coins we are talking about,
    /// the denomination must be unique amongst all PoolAssets for this pool.
    #[prost(message, optional, tag = "1")]
    pub token: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// Weight that is not normalized. This weight must be less than 2^50
    #[prost(string, tag = "2")]
    pub weight: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub id: u64,
    #[prost(message, optional, tag = "3")]
    pub pool_params: ::core::option::Option<PoolParams>,
    /// This string specifies who will govern the pool in the future.
    /// Valid forms of this are:
    /// {token name},{duration}
    /// {duration}
    /// where {token name} if specified is the token which determines the
    /// governor, and if not specified is the LP token for this pool.duration is
    /// a time specified as 0w,1w,2w, etc. which specifies how long the token
    /// would need to be locked up to count in governance. 0w means no lockup.
    /// TODO: Further improve these docs
    #[prost(string, tag = "4")]
    pub future_pool_governor: ::prost::alloc::string::String,
    /// sum of all LP tokens sent out
    #[prost(message, optional, tag = "5")]
    pub total_shares: ::core::option::Option<
        super::super::super::super::cosmos::base::v1beta1::Coin,
    >,
    /// These are assumed to be sorted by denomiation.
    /// They contain the pool asset and the information about the weight
    #[prost(message, repeated, tag = "6")]
    pub pool_assets: ::prost::alloc::vec::Vec<PoolAsset>,
    /// sum of all non-normalized pool weights
    #[prost(string, tag = "7")]
    pub total_weight: ::prost::alloc::string::String,
}