use serde::Serialize;

use crate::chat::message::Message;

#[derive(Serialize, Debug)]
pub struct PlayerSample {
    pub name: String,
    pub id: String
}

#[derive(Serialize, Debug)]
pub struct ServerListPingPlayers {
    pub max: i32,
    pub online: i32,
    pub sample: Vec<PlayerSample>
}

#[derive(Serialize, Debug)]
pub struct ServerListPingVersion {
    pub name: String,
    pub protocol: i32
}

#[derive(Serialize, Debug)]
pub struct ServerListPing {
    pub version: ServerListPingVersion,
    pub players: ServerListPingPlayers,
    pub description: Message,
    pub favicon: String,
    pub previewsChat: bool
}