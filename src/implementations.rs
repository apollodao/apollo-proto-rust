use crate::cosmos::base::v1beta1::Coin;

impl From<cosmwasm_std::Coin> for Coin {
    fn from(coin: cosmwasm_std::Coin) -> Self {
        Coin {
            denom: coin.denom,
            amount: coin.amount.to_string(),
        }
    }
}
