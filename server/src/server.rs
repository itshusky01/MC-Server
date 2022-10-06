use async_std::io::ReadExt;
use async_std::net::{TcpListener, TcpStream};

use futures::AsyncWriteExt;
use futures::stream::StreamExt;

use std::io::{Result, Error, ErrorKind};
use log::{info, warn, error};

use crate::packet::*;
use crate::session::Session;
use crate::config::StartupConfig;

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
                    Ok(mut v) => { 
                        if let Err(e) = self.handle(&mut v).await {
                            // error!("{}", e);
                            v.close().await;
                        }
                    }
                }
            }).await;
    }

    async fn handle(&self, stream: &mut TcpStream) -> Result<()> {
        info!("Got a connection from {}", stream.peer_addr().unwrap().to_string());
        
        match stream.read_packet().await {
            Err(e) => return Err(e),
            Ok(packet) => {
                if let Packet::SLP() = packet {
                    // Server List Ping
                }
            }
        }
        
        Ok(())
    }
}
