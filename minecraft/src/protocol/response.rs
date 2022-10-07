use std::io::{Result};

pub trait PacketSerializable {
    fn serialize(&self) -> Vec<u8>;
}

