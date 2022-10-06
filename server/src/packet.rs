use async_std::{io::{Read, ReadExt}, net::TcpStream};
use async_trait::async_trait;
use log::info;

use std::io::{Result, Error, ErrorKind};

pub enum Packet {
    LegacyPing(),

    Handshake(),
    SLP(),
    Ping(),
    Unknown()
}

impl Packet {
    pub fn parse(bytes: Vec<u8>) -> Self {
        let id = bytes[0];
        info!("len: {}", bytes.len());

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

async fn read_varint32(s: &mut (impl Read + Unpin)) -> Result<i32> {
    let mut value = 0i32;
    let mut shift = 0u32;

    let mut buf = [0u8;1];
    let mut byte = 0u8;

    loop {
        if let Err(e) = s.read(&mut buf).await {
            return Err(e);
        }
        byte = buf[0];

        value |= (byte & 127 as u8).wrapping_shl(shift * 7) as i32;
        shift += 1;

        if shift > 5 {
            return  Err(Error::new(ErrorKind::Other, "VarInt too big"));
        }
                
        if (byte & 128) != 128 {
            break;
        }
    }

    Ok(value)
}

#[async_trait]
pub trait PacketRead : Read + Unpin {
    async fn read_packet(&mut self) -> Result<Packet> {
        let buf = read_varint32(&mut self).await;

        if let Err(e) = buf {
            return Err(e);
        }

        match buf.unwrap() {
            0 => Err(Error::new(ErrorKind::Other, "EOF")),
            254 => Ok(Packet::LegacyPing()),
            i => {
                let mut buf=  vec![0; i as usize];
                if let Err(e) = self.read(&mut buf).await {
                    return Err(e);
                }

                Ok(Packet::parse(buf))
            }
        }
    }
}

impl PacketRead for TcpStream {}