use axum::extract::ws::{WebSocket, Message};
use futures::SinkExt;
use futures::stream::SplitSink;

use jasshaus_comm::socket_message::SocketMessage;

pub type WsWriter = SplitSink<WebSocket, Message>;

#[derive(Default)]
pub struct Client {
    pub connected: bool,
    pub player_id: u8,
    pub ws: Option< WsWriter >,
}

impl Client {
    pub const fn new(player_id: u8) -> Self {
        Client {
            connected: false,
            player_id,
            ws: None,
        }
    }

    pub fn connect(&mut self, ws_tx: WsWriter){
        self.ws = Some(ws_tx);
        self.connected = true;
    }

    pub fn disconnect(&mut self) {
        self.ws = None;
        self.connected = false;
    }

    pub async fn send(&mut self, data: SocketMessage) {
        if self.ws.is_none() { return; }

        let jsonstr = serde_json::to_string(&data).unwrap();
        let msg = Message::Text(jsonstr);
        let unwrapped_ws = self.ws.as_mut().unwrap();

        if let Err(e) = unwrapped_ws.send( msg ).await {
            eprintln!("Error sending data: {:?}", e);
        }
    }
}
