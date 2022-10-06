use async_std::{io::{Read, ReadExt}, net::TcpStream};
use async_trait::async_trait;

use std::io::{Result, Error, ErrorKind};

pub enum Packet {
    Handshake(),
    SLP(),
    Ping(),
    Unknown()
}

impl Packet {
    pub fn parse(bytes: Vec<u8>) -> Self {
        let id = bytes[0];
        
        match id {
            0x00 => {
                if bytes.len() == 1 {
                    Packet::SLP()
                } else {
                    Packet::Handshake()
                }
            },
            default => {
                Packet::Unknown()
            }
        }
    }
}


async fn read_varint_u32(s: &mut (impl Read + Unpin)) -> Result<u32> {
    let mut buffer = vec![0u8; 1];
    let mut value = 0_u32;
    let mut shift = 0_u32;
    let mut tmp = 0_u8;
    loop {
        match s.read(&mut buffer).await {
            Ok(cnt) => {
                if cnt == 1 {
                    tmp = buffer[0];
                } else {
                    return Err(Error::new(ErrorKind::Other, "EOF"));
                }
            },
            Err(e) => return Err(e)
        }
        value |= ((tmp & 127) as u32) << shift;
        if (tmp & (0b00000001 << 7)) != 0 {
            shift += 7;
        } else {
            return Ok(value);
        }
    }
}

#[async_trait]
pub trait PacketRead : Read + Unpin {
    async fn read_packet(&mut self) -> Result<Packet> {
        let len = read_varint_u32(&mut self).await;


        if let Err(e) = len {
            return Err(e);
        }
        let len = len.unwrap();
        if len == 0 {
            return Err(Error::new(ErrorKind::Other, "EOF"));
        }

        let mut buf=  vec![0; len as usize];
        if let Err(e) = self.read(&mut buf).await {
            return Err(e);
        }

        Ok(Packet::parse(buf))
    }
}

impl PacketRead for TcpStream {}