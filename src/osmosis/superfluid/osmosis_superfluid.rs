/// SuperfluidAsset stores the pair of superfluid asset type and denom pair
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidAsset {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(enumeration="SuperfluidAssetType", tag="2")]
    pub asset_type: i32,
}
/// SuperfluidIntermediaryAccount takes the role of intermediary between LP token
/// and OSMO tokens for superfluid staking
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidIntermediaryAccount {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub val_addr: ::prost::alloc::string::String,
    /// perpetual gauge for rewards distribution
    #[prost(uint64, tag="3")]
    pub gauge_id: u64,
}
/// The Osmo-Equivalent-Multiplier Record for epoch N refers to the osmo worth we
/// treat an LP share as having, for all of epoch N. Eventually this is intended
/// to be set as the Time-weighted-average-osmo-backing for the entire duration
/// of epoch N-1. (Thereby locking whats in use for epoch N as based on the prior
/// epochs rewards) However for now, this is not the TWAP but instead the spot
/// price at the boundary.  For different types of assets in the future, it could
/// change.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsmoEquivalentMultiplierRecord {
    #[prost(int64, tag="1")]
    pub epoch_number: i64,
    /// superfluid asset denom, can be LP token or native token
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub multiplier: ::prost::alloc::string::String,
}
/// SuperfluidDelegationRecord takes the role of intermediary between LP token
/// and OSMO tokens for superfluid staking
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationRecord {
    #[prost(string, tag="1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub delegation_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag="4")]
    pub equivalent_staked_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockIdIntermediaryAccountConnection {
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
    #[prost(string, tag="2")]
    pub intermediary_account: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnpoolWhitelistedPools {
    #[prost(uint64, repeated, tag="1")]
    pub ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SuperfluidAssetType {
    Native = 0,
    /// SuperfluidAssetTypeLendingShare = 2; // for now not exist
    LpShare = 1,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidDelegate {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub lock_id: u64,
    #[prost(string, tag="3")]
    pub val_addr: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidDelegateResponse {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidUndelegate {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub lock_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidUndelegateResponse {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidUnbondLock {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub lock_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidUnbondLockResponse {
}
// message MsgSuperfluidRedelegate {
//   string sender = 1 [ (gogoproto.moretags) = "yaml:\"sender\"" ];
//   uint64 lock_id = 2;
//   string new_val_addr = 3;
// }
// message MsgSuperfluidRedelegateResponse {}

/// MsgLockAndSuperfluidDelegate locks coins with the unbonding period duration,
/// and then does a superfluid lock from the newly created lockup, to the
/// specified validator addr.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLockAndSuperfluidDelegate {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="3")]
    pub val_addr: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLockAndSuperfluidDelegateResponse {
    #[prost(uint64, tag="1")]
    pub id: u64,
}
/// MsgUnPoolWhitelistedPool Unpools every lock the sender has, that is
/// associated with pool pool_id. If pool_id is not approved for unpooling by
/// governance, this is a no-op. Unpooling takes the locked gamm shares, and runs
/// "ExitPool" on it, to get the constituent tokens. e.g. z gamm/pool/1 tokens
/// ExitPools into constituent tokens x uatom, y uosmo. Then it creates a new
/// lock for every constituent token, with the duration associated with the lock.
/// If the lock was unbonding, the new lockup durations should be the time left
/// until unbond completion.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnPoolWhitelistedPool {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnPoolWhitelistedPoolResponse {
    #[prost(uint64, repeated, tag="1")]
    pub exited_lock_ids: ::prost::alloc::vec::Vec<u64>,
}