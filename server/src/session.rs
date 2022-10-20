use async_std::{net::TcpStream, task::block_on};

use log::info;
use minecraft::types::PlayerProfile;
use minecraft::protocol::server::request::{handshake::*, ping::*};
use minecraft::protocol::server::response::pong::*;
use minecraft::protocol::server::packet::{ReadPacket, DedeserializePacket};
use minecraft::util::binary::*;
use minecraft::error::{Result, Error, ErrorKind};

use std::collections::HashMap;

use crate::conn::Connection;
use crate::server::Server;

pub struct SessionManager {
    server: &'static Server,
    sessions: HashMap<String, Session>,
    limit: i32
}

impl SessionManager {
    pub fn new(max_sessions: i32, server: &'static Server) -> Self {
        Self {
            server,
            sessions: HashMap::new(),
            limit: max_sessions
        }
    }

    pub fn imcoming(&mut self, mut conn: Connection) {

    }
}

pub struct Session {
    conn: Connection,
    mgr: &SessionManager,

    profile: PlayerProfile
}

impl Session {
    pub fn new<'a>(mut conn: Connection, server: &'a Server) -> Result<Session> {
        let packet = conn.read_packet()?;
        let handshake = HandshakePacket::deserialize(packet)?;

        match handshake.status {
            HandshakeStatus::Status => for _ in 0..2  {
                match PingPacket::deserialize(conn.read_packet()?)? {
                    PingPacket::Ping { payload } => {
                        conn.write_packet(PongPacket::Pong { payload })
                    },
                    PingPacket::ServerListPing() => {
                        conn.write_packet(PongPacket::ServerListPing { data: server.ping()} )
                    }
                }
            },
            HandshakeStatus::Login => {
                let packet = conn.read_packet()?;
                if packet.id != 0x00 {
                    return Err(Error::new(ErrorKind::BrokenPacket, ""));
                }

                let mut reader = BinaryReader::new(packet.data);
                let name = reader.read_string()?;

                info!("Player {} trying join game", name);

                if server.online_mode() {
                    // 微软账户登录, Not implement yet
                } else {
                    return Ok(Self {
                        conn,
                        profile: PlayerProfile{ name },
                    });

                    // next -> handle
                }
            }
        };

        Err(Error::new(ErrorKind::Other, "No error"))
    }

    pub async fn handle() -> Result<()>{
        Ok(())
    }
}
