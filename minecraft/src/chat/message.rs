use serde::Serialize;
use crate::util::binary::{ToBytes, BinaryWriter};

#[derive(Serialize, Debug)]
pub struct Message {
    pub text: String
}

impl ToBytes for Message {
    fn to_bytes(&self) -> Vec<u8> {
        let mut writer = BinaryWriter::new();
        writer.write(serde_json::to_string(self).unwrap());
        writer.buffer()
    }
}

impl ToBytes for &Message {
    fn to_bytes(&self) -> Vec<u8> {
        (*(*self).to_bytes()).to_vec()
    }
}