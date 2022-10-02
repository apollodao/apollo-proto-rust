use cosmwasm_std::Binary;
use prost::{DecodeError, Message};

pub fn encode<T: Message>(proto_msg: T) -> Binary {
    Binary::from(proto_msg.encode_to_vec())
}

pub fn decode<T: Message + Default>(binary_msg: &Binary) -> Result<T, DecodeError> {
    Message::decode(binary_msg.as_slice())
}
