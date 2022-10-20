/// PeriodLock is a single lock unit by period defined by the x/lockup module.
/// It's a record of a locked coin at a specific time. It stores owner, duration,
/// unlock time and the number of coins locked. A state of a period lock is
/// created upon lock creation, and deleted once the lock has been matured after
/// the `duration` has passed since unbonding started.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeriodLock {
    /// ID is the unique id of the lock.
    /// The ID of the lock is decided upon lock creation, incrementing by 1 for
    /// every lock.
    #[prost(uint64, tag="1")]
    pub id: u64,
    /// Owner is the account address of the lock owner.
    /// Only the owner can modify the state of the lock.
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    /// Duration is the time needed for a lock to mature after unlocking has
    /// started.
    #[prost(message, optional, tag="3")]
    pub duration: ::core::option::Option<super::super::super::google::protobuf::Duration>,
    /// EndTime refers to the time at which the lock would mature and get deleted.
    /// This value is first initialized when an unlock has started for the lock,
    /// end time being block time + duration.
    #[prost(message, optional, tag="4")]
    pub end_time: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    /// Coins are the tokens locked within the lock, kept in the module account.
    #[prost(message, repeated, tag="5")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryCondition is a struct used for querying locks upon different conditions.
/// Duration field and timestamp fields could be optional, depending on the
/// LockQueryType.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCondition {
    /// LockQueryType is a type of lock query, ByLockDuration | ByLockTime
    #[prost(enumeration="LockQueryType", tag="1")]
    pub lock_query_type: i32,
    /// Denom represents the token denomination we are looking to lock up
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    /// Duration is used to query locks with longer duration than the specified
    /// duration. Duration field must not be nil when the lock query type is
    /// `ByLockDuration`.
    #[prost(message, optional, tag="3")]
    pub duration: ::core::option::Option<super::super::super::google::protobuf::Duration>,
    /// Timestamp is used by locks started before the specified duration.
    /// Timestamp field must not be nil when the lock query type is `ByLockTime`.
    /// Querying locks with timestamp is currently not implemented.
    #[prost(message, optional, tag="4")]
    pub timestamp: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
}
/// SyntheticLock is creating virtual lockup where new denom is combination of
/// original denom and synthetic suffix. At the time of synthetic lockup creation
/// and deletion, accumulation store is also being updated and on querier side,
/// they can query as freely as native lockup.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyntheticLock {
    /// Underlying Lock ID is the underlying native lock's id for this synthetic
    /// lockup. A synthetic lock MUST have an underlying lock.
    #[prost(uint64, tag="1")]
    pub underlying_lock_id: u64,
    /// SynthDenom is the synthetic denom that is a combination of
    /// gamm share + bonding status + validator address.
    #[prost(string, tag="2")]
    pub synth_denom: ::prost::alloc::string::String,
    /// used for unbonding synthetic lockups, for active synthetic lockups, this
    /// value is set to uninitialized value
    #[prost(message, optional, tag="3")]
    pub end_time: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    /// Duration is the duration for a synthetic lock to mature
    /// at the point of unbonding has started.
    #[prost(message, optional, tag="4")]
    pub duration: ::core::option::Option<super::super::super::google::protobuf::Duration>,
}
/// LockQueryType defines the type of the lock query that can
/// either be by duration or start time of the lock.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LockQueryType {
    ByDuration = 0,
    ByTime = 1,
}
impl LockQueryType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LockQueryType::ByDuration => "ByDuration",
            LockQueryType::ByTime => "ByTime",
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLockTokens {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub duration: ::core::option::Option<super::super::super::google::protobuf::Duration>,
    #[prost(message, repeated, tag="3")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLockTokensResponse {
    #[prost(uint64, tag="1")]
    pub id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBeginUnlockingAll {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBeginUnlockingAllResponse {
    #[prost(message, repeated, tag="1")]
    pub unlocks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBeginUnlocking {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub id: u64,
    /// Amount of unlocking coins. Unlock all if not set.
    #[prost(message, repeated, tag="3")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBeginUnlockingResponse {
    #[prost(bool, tag="1")]
    pub success: bool,
}
/// MsgExtendLockup extends the existing lockup's duration.
/// The new duration is longer than the original.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExtendLockup {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub id: u64,
    /// duration to be set. fails if lower than the current duration, or is
    /// unlocking
    #[prost(message, optional, tag="3")]
    pub duration: ::core::option::Option<super::super::super::google::protobuf::Duration>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExtendLockupResponse {
    #[prost(bool, tag="1")]
    pub success: bool,
}
/// MsgForceUnlock unlocks locks immediately for
/// addresses registered via governance.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgForceUnlock {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub id: u64,
    /// Amount of unlocking coins. Unlock all if not set.
    #[prost(message, repeated, tag="3")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgForceUnlockResponse {
    #[prost(bool, tag="1")]
    pub success: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleBalanceRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleBalanceResponse {
    #[prost(message, repeated, tag="1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleLockedAmountRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleLockedAmountResponse {
    #[prost(message, repeated, tag="1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockableCoinsRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockableCoinsResponse {
    #[prost(message, repeated, tag="1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockingCoinsRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockingCoinsResponse {
    #[prost(message, repeated, tag="1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedCoinsRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedCoinsResponse {
    #[prost(message, repeated, tag="1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeResponse {
    #[prost(message, repeated, tag="1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeNotUnlockingOnlyRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeNotUnlockingOnlyResponse {
    #[prost(message, repeated, tag="1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockedBeforeTimeRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockedBeforeTimeResponse {
    #[prost(message, repeated, tag="1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeDenomRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    #[prost(string, tag="3")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeDenomResponse {
    #[prost(message, repeated, tag="1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedDenomRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub duration: ::core::option::Option<super::super::super::google::protobuf::Duration>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedDenomResponse {
    #[prost(string, tag="1")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedRequest {
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedResponse {
    #[prost(message, optional, tag="1")]
    pub lock: ::core::option::Option<PeriodLock>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyntheticLockupsByLockupIdRequest {
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyntheticLockupsByLockupIdResponse {
    #[prost(message, repeated, tag="1")]
    pub synthetic_locks: ::prost::alloc::vec::Vec<SyntheticLock>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub duration: ::core::option::Option<super::super::super::google::protobuf::Duration>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationResponse {
    #[prost(message, repeated, tag="1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedDurationRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub duration: ::core::option::Option<super::super::super::google::protobuf::Duration>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedDurationResponse {
    #[prost(message, repeated, tag="1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationNotUnlockingOnlyRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub duration: ::core::option::Option<super::super::super::google::protobuf::Duration>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationNotUnlockingOnlyResponse {
    #[prost(message, repeated, tag="1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationDenomRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub duration: ::core::option::Option<super::super::super::google::protobuf::Duration>,
    #[prost(string, tag="3")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationDenomResponse {
    #[prost(message, repeated, tag="1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}