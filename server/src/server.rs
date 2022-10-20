use async_std::net::{TcpListener};
use futures::stream::StreamExt;
use minecraft::chat::message::Message;
use minecraft::protocol::common::*;
use std::io::{Result};
use log::{error};

use crate::conn::Connection;
use crate::session::{Session, SessionManager};
use crate::config::StartupConfig;

pub struct Server {
    listener: TcpListener,
    session_manager: Option<SessionManager>,

    max_players: i32,
    online_mode: bool,
}

impl Server{
    pub async fn new(config: StartupConfig) -> Result<Server> {
        let listener = TcpListener::bind(config.address.clone()).await?;
        let mut server = Self {
            listener,
            max_players: config.max_players,
            online_mode: false,
            session_manager: None,
        };

        server.session_manager = Some(SessionManager::new(config.max_players, &server));
        Ok(server)
    }

    pub async fn run(&self) {
        let listener = &self.listener;
        listener
            .incoming()
            .for_each_concurrent(None, |stream| async move {
                match stream {
                    Err(err) => error!("{}", err),
                    Ok(v) => { 
                        if let Err(e) = Session::new(Connection::new(v), self) {}
                    }
                }
            }).await;
    }

    pub fn online_mode(&self) -> bool {
        self.online_mode
    }
    
    pub fn ping(&self) -> ServerListPing {
        ServerListPing {
            version: ServerListPingVersion {
                name: String::from("1.12.2"),
                protocol: 340,
            },
            players: ServerListPingPlayers {
                max: 1024,
                online: 1023,
                sample: Vec::new(),
            },
            description:Message {
                text: String::from("This is a Minecraft server")
            },
            favicon: String::from("data:image/png;base64,<data>"),
            previewsChat: true
        }
    }
}