/// ===================== MsgJoinPool
/// This is really MsgJoinPoolNoSwap
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinPool {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
    #[prost(string, tag="3")]
    pub share_out_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub token_in_maxs: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinPoolResponse {
}
/// ===================== MsgExitPool
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitPool {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
    #[prost(string, tag="3")]
    pub share_in_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub token_out_mins: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitPoolResponse {
}
/// ===================== MsgSwapExactAmountIn
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapAmountInRoute {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
    #[prost(string, tag="2")]
    pub token_out_denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountIn {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
    #[prost(message, optional, tag="3")]
    pub token_in: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="4")]
    pub token_out_min_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountInResponse {
    #[prost(string, tag="1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgSwapExactAmountOut
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapAmountOutRoute {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
    #[prost(string, tag="2")]
    pub token_in_denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountOut {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
    #[prost(string, tag="3")]
    pub token_in_max_amount: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub token_out: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountOutResponse {
    #[prost(string, tag="1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// ===================== MsgJoinSwapExternAmountIn
/// TODO: Rename to MsgJoinSwapExactAmountIn
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinSwapExternAmountIn {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
    #[prost(message, optional, tag="3")]
    pub token_in: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// repeated cosmos.base.v1beta1.Coin tokensIn = 5 [
    ///   (gogoproto.moretags) = "yaml:\"tokens_in\"",
    ///   (gogoproto.nullable) = false
    /// ];
    #[prost(string, tag="4")]
    pub share_out_min_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinSwapExternAmountInResponse {
    #[prost(string, tag="1")]
    pub share_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgJoinSwapShareAmountOut
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinSwapShareAmountOut {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
    #[prost(string, tag="3")]
    pub token_in_denom: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub share_out_amount: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub token_in_max_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinSwapShareAmountOutResponse {
    #[prost(string, tag="1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// ===================== MsgExitSwapShareAmountIn
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitSwapShareAmountIn {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
    #[prost(string, tag="3")]
    pub token_out_denom: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub share_in_amount: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub token_out_min_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitSwapShareAmountInResponse {
    #[prost(string, tag="1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgExitSwapExternAmountOut
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitSwapExternAmountOut {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
    #[prost(message, optional, tag="3")]
    pub token_out: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="4")]
    pub share_in_max_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitSwapExternAmountOutResponse {
    #[prost(string, tag="1")]
    pub share_in_amount: ::prost::alloc::string::String,
}