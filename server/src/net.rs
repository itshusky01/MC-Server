use std::io::{Result, Error, ErrorKind};

use async_std::{net::TcpStream, task::block_on, io::{ReadExt, WriteExt}};
use log::info;
use minecraft::{protocol::{packet::Packet, response::PacketSerializable}, util::writer::BinaryWriter};

pub struct Connection {
    pub stream: TcpStream,
}

impl Connection  {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream
        }
    }

    pub fn read_packet(&mut self) -> Result<Packet> {
        let len = self.read_packet_length();
        if len == -1 || len == 0 {
            return Err(Error::new(ErrorKind::Other, "EOF"))
        }

        if len >= 10240 /*limit*/ {
            return Err(Error::new(ErrorKind::Other, "Out of limit"))
        }

        let llen = BinaryWriter::varint_len(len as i32);

        let mut buf = vec![0u8; len as usize + llen];
        // buf.append(other)
        match block_on(self.stream.read(&mut buf[llen..])) {
            Err(e) => return Err(e),
            Ok(0) => return Err(Error::new(ErrorKind::Other, "EOF")),
            Ok(_) => {}
        }
        Packet::parse(buf)
        
    }

    pub fn write_packet(&mut self, packet: impl PacketSerializable) -> Result<()> {
        let bytes = packet.serialize();
        info!("{:?}", bytes);
        match block_on(self.stream.write(&bytes)) {
            Err(e) => Err(e),
            Ok(0) => Err(Error::new(ErrorKind::Other, "EOF")),
            Ok(_) => Ok(())
        }
    }

    fn read_packet_length(&mut self) -> isize {
        let mut value = 0_i32;
        let mut shift = 0u32;
        let mut buf = [0u8;1];
        loop {
            match block_on(self.stream.read(&mut buf)) {
                Ok(1) => {
                    let byte = buf[0];
                    value |= (byte as i32 & 127).wrapping_shl(shift * 7);
                    shift += 1;
                    if shift > 5 {
                        return -1;
                    }
                        
                    if (byte as u8 & 128) != 128 {
                        break;
                    }
                },
                Err(_) | Ok(_) => return -1
            }
        }

        value as isize
    }
}