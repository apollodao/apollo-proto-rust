use cosmwasm_std::Binary;
use prost::Message;

pub fn encode<T: Message>(proto_msg: T) -> Binary {
    Binary::from(proto_msg.encode_to_vec())
}
