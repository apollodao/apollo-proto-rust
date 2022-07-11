use std::fmt::{Display, Formatter};

pub enum OsmosisTypeURLs {
    CreateDenom,
    Mint,
    Burn,
    SwapExactAmountIn,
    JoinPool,
    ExitPool,
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
        }
    }
}
