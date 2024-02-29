use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
    routing::get,
      Router,
};

use futures::StreamExt;
use jasshaus_comm::socket_message::SocketMessage;

#[macro_use]
mod log;
mod room;


async fn handle_websocket(ws: WebSocket, room: Arc<Mutex<room::Room>>) {
  let (ws_tx, mut ws_rx) = ws.split();
  let client_id: u8 = room.lock().await
    .register(ws_tx).await
    .expect("The room was already full!");

  debug!("Client[{}] connected!", client_id);

  while let Some(message) = ws_rx.next().await {
    if let Err(e) = message {
      error!("{:?}", e);
      break;
    }

    if let Message::Text(string) = message.unwrap() {
      let msg: SocketMessage = serde_json::from_str( string.as_str() ).expect("Could not parse SocketMessage from JSON!");

      room.lock().await
        .handle_input(msg, client_id).await;
    } else {
      // TODO handle closing and other codes etc...
      error!("Could not extract message!");
    }
  }

  room.lock().await
    .unregister(client_id).await;
  debug!("Client[{}] disconnected!", client_id);
}

#[tokio::main]
async fn main() {
  let room = Arc::new( Mutex::new( room::Room::new() ));

  let app = Router::new()
    .route("/ws", get(|ws: WebSocketUpgrade| async move {
      ws.on_upgrade(|ws: WebSocket| async move {
        handle_websocket(ws, room.clone()).await;
      })
    }));

  let addr = &"127.0.0.1:7999".parse().unwrap();
  axum::Server::bind(addr)
      .serve(app.into_make_service())
      .await
      .expect("Can not start server!");
}
