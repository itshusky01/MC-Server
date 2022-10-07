use async_std::io::{ReadExt, Read, Write};
use async_std::net::{TcpListener, TcpStream};

use async_trait::async_trait;
use futures::AsyncWriteExt;
use futures::stream::StreamExt;
use minecraft::protocol::handshake::*;
use minecraft::protocol::ping::ResponsePacket;
use minecraft::protocol::response::PacketSerializable;

use std::io::{Result, Error, ErrorKind};
use log::{info, warn, error};

use crate::net::Connection;
use crate::session::Session;
use crate::config::StartupConfig;

use minecraft::protocol::packet::*;

pub struct Server {
    listener: TcpListener,
    sessions: Vec<Session>,

    name: String,
    version: String,

    max_players: i32,
}

impl Server{
    pub async fn new(config: StartupConfig) -> Result<Self> {
        let listener = TcpListener::bind(config.address.clone()).await;
        return match listener {
            Err(err) => Err(err),
            Ok(v) => Ok(Self {
                listener: v,
                sessions: Vec::new(),
    
                name: config.name.clone(),
                version: String::from("1.12.2"),
                max_players: config.max_players
            })
        }
    }

    pub async fn run(&self) {
        let listener = &self.listener;
        listener
            .incoming()
            .for_each_concurrent(None, |stream| async move {
                match stream {
                    Err(err) => error!("{}", err),
                    Ok(v) => { 
                        let conn = Connection::new(v);
                        if let Err(e) = self.incoming(conn).await {
                            error!("{}", e);
                        }
                    }
                }
            }).await;
    }

    async fn incoming(&self, mut conn: Connection) -> Result<()> {
        info!("Got a connection from {}", conn.stream.peer_addr().unwrap().to_string());
        
        let packet;
        match conn.read_packet() {
            Err(e) => return Err(e),
            Ok(v) => packet = v
        }

        let handshake;
        match Handshake::parse(&packet) {
            Err(e) => return Err(e),
            Ok(v) => handshake = v
        }

        info!("{:?}", handshake);
        match handshake.status {
            HandshakeStatus::Status => self.slp_handle(conn),
            HandshakeStatus::Login => todo!(),
        };

        Ok(())
    }

    fn slp_handle(&self, mut conn: Connection) {
        for _ in 0..2  {
            if let Ok(p) = conn.read_packet() {
                match p.id {
                    0x00 => {
                        info!("SLP");
                    },
                    0x01 => {
                        let res = ResponsePacket::Pong { payload: 0 };
                        conn.write_packet(res);
                    },
                    _ => {}
                }
            }
        }
    }
}