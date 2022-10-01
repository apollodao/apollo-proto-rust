/// PoolParams defined the parameters that will be managed by the pool
/// governance in the future. This params are not managed by the chain
/// governance. Instead they will be managed by the token holders of the pool.
/// The pool's token holders are specified in future_pool_governor.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolParams {
    #[prost(string, tag="1")]
    pub swap_fee: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub exit_fee: ::prost::alloc::string::String,
}
/// Pool is the stableswap Pool struct
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub id: u64,
    #[prost(message, optional, tag="3")]
    pub pool_params: ::core::option::Option<PoolParams>,
    /// This string specifies who will govern the pool in the future.
    /// Valid forms of this are:
    /// {token name},{duration}
    /// {duration}
    /// where {token name} if specified is the token which determines the
    /// governor, and if not specified is the LP token for this pool.duration is
    /// a time specified as 0w,1w,2w, etc. which specifies how long the token
    /// would need to be locked up to count in governance. 0w means no lockup.
    #[prost(string, tag="4")]
    pub future_pool_governor: ::prost::alloc::string::String,
    /// sum of all LP shares
    #[prost(message, optional, tag="5")]
    pub total_shares: ::core::option::Option<super::super::super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// assets in the pool
    #[prost(message, repeated, tag="6")]
    pub pool_liquidity: ::prost::alloc::vec::Vec<super::super::super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// for calculation amognst assets with different precisions
    #[prost(uint64, repeated, packed="false", tag="7")]
    pub scaling_factor: ::prost::alloc::vec::Vec<u64>,
    /// scaling_factor_controller is the address can adjust pool scaling factors
    #[prost(string, tag="8")]
    pub scaling_factor_controller: ::prost::alloc::string::String,
}