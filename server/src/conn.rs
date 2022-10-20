use async_std::{net::TcpStream, task::block_on, io::{ReadExt, WriteExt}};
use minecraft::{util::binary::*, error::*, protocol::server::packet::{ReadPacket, Packet, SerializePacket}};

pub struct Connection {
    stream: TcpStream
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub fn tcp_stream(&self) -> &TcpStream {
        &self.stream
    }

    pub fn write_packet(&mut self, packet: impl SerializePacket) {
        block_on(self.stream.write(&packet.serialize()));
    }
}

impl ReadN for Connection {
    fn _read(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; n];
        match block_on(self.stream.read(&mut buf)) {
            Err(e) => Err(Error::new(ErrorKind::Other, &e.to_string())),
            Ok(0) => Err(Error::new(ErrorKind::EOF, "EOF")),
            Ok(_) => Ok(buf)
        }
    }
}

impl ReadVarInt for Connection {}
impl ReadPacket for Connection {}