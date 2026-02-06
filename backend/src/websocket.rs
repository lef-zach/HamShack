use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{SinkExt, StreamExt};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::broadcast;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<crate::AppState>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<crate::AppState>) {
    // Create broadcast channel for this socket
    let (_tx, mut rx): (broadcast::Sender<String>, _) = tokio::sync::broadcast::channel(100);

    // Send initial status
    let sdr_manager = state.sdr_manager.lock().await;
    let status = sdr_manager.get_status();
    let status_json = json!({
        "type": "sdr_status",
        "data": status
    });
    
    if let Err(e) = socket.send(Message::Text(status_json.to_string().into())).await {
        println!("WebSocket send error: {}", e);
        return;
    }

    // Handle incoming messages and broadcast
    let (mut sender, mut receiver) = socket.split();

    tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            if let Message::Text(text) = message {
                // Handle client messages (e.g., frequency changes)
                println!("Received WebSocket message: {}", text);
            }
        }
    });

    // Broadcast messages to client
    while let Ok(message) = rx.recv().await {
        if let Err(e) = sender.send(Message::Text(message.into())).await {
            println!("WebSocket broadcast error: {}", e);
            break;
        }
    }
}