use std::fmt::{Display, Formatter};

pub enum OsmosisTypeURLs {
    // Execute Msgs
    CreateDenom,
    Mint,
    Burn,
    SwapExactAmountIn,
    JoinPool,
    ExitPool,
    JoinSwapExternAmountIn,
    JoinSwapShareAmountOut,
    ExitSwapShareAmountIn,
    ExitSwapExternAmountOut,
    BondLP,
    UnBondLP,
    SuperfluidBondLP,
    SuperfluidUnBondLP,

    // Query Msgs
    QueryPool,
    QueryTotalLiquidity,
    QueryTotalPoolLiquidity,
    QueryPoolParams,
    QueryAccountLockedLongerDurationNotUnlockingOnly { owner: String },
}

impl Display for OsmosisTypeURLs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OsmosisTypeURLs::CreateDenom => {
                write!(f, "/osmosis.tokenfactory.v1beta1.MsgCreateDenom")
            }
            OsmosisTypeURLs::Mint => write!(f, "/osmosis.tokenfactory.v1beta1.MsgMint"),
            OsmosisTypeURLs::Burn => write!(f, "/osmosis.tokenfactory.v1beta1.MsgBurn"),
            OsmosisTypeURLs::SwapExactAmountIn => {
                write!(f, "/osmosis.gamm.v1beta1.MsgSwapExactAmountIn")
            }
            OsmosisTypeURLs::JoinPool => {
                write!(f, "/osmosis.gamm.v1beta1.MsgJoinPool")
            }
            OsmosisTypeURLs::ExitPool => {
                write!(f, "/osmosis.gamm.v1beta1.MsgExitPool")
            }
            OsmosisTypeURLs::BondLP => {
                write!(f, "/osmosis.lockup.MsgLockTokens")
            }
            OsmosisTypeURLs::UnBondLP => {
                write!(f, "/osmosis.lockup.MsgBeginUnlocking")
            }
            OsmosisTypeURLs::SuperfluidBondLP => {
                write!(f, "/osmosis.superfluid.MsgLockAndSuperfluidDelegate")
            }
            OsmosisTypeURLs::SuperfluidUnBondLP => {
                write!(f, "/osmosis.superfluid.MsgSuperfluidUnbondLock")
            }
            OsmosisTypeURLs::JoinSwapExternAmountIn => {
                write!(f, "/osmosis.gamm.v1beta1.MsgJoinSwapExternAmountIn")
            }
            OsmosisTypeURLs::JoinSwapShareAmountOut => {
                write!(f, "/osmosis.gamm.v1beta1.MsgJoinSwapShareAmountOut")
            }
            OsmosisTypeURLs::ExitSwapShareAmountIn => {
                write!(f, "/osmosis.gamm.v1beta1.MsgExitSwapShareAmountIn")
            }
            OsmosisTypeURLs::ExitSwapExternAmountOut => {
                write!(f, "/osmosis.gamm.v1beta1.MsgExitSwapExternAmountOut")
            }

            // Queries
            OsmosisTypeURLs::QueryPool => {
                write!(f, "/osmosis.gamm.v1beta1.Query/Pool")
            }
            OsmosisTypeURLs::QueryTotalLiquidity => {
                write!(f, "/osmosis.gamm.v1beta1.Query/TotalLiquidity")
            }
            OsmosisTypeURLs::QueryTotalPoolLiquidity => {
                write!(f, "/osmosis.gamm.v1beta1.Query/TotalPoolLiquidity")
            }
            OsmosisTypeURLs::QueryPoolParams => {
                write!(f, "/osmosis.gamm.v1beta1.Query/PoolParams")
            }
            OsmosisTypeURLs::QueryAccountLockedLongerDurationNotUnlockingOnly { owner } => write!(
                f,
                "/osmosis/lockup/v1beta1/account_locked_longer_duration_not_unlocking_only/{owner}"
            ),
        }
    }
}
