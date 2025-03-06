//! WebSocket API implementation for the Kraken API

use futures::{SinkExt, StreamExt};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

use crate::client::KrakenClient;
use crate::error::{Error, Result};
use crate::models::websocket::{WebSocketMessage, WebSocketSubscriptionRequest, WebSocketUnsubscriptionRequest};

/// WebSocket API
pub struct WebSocketApi {
    /// WebSocket URL
    ws_url: String,
    
    /// Message sender
    tx: Option<mpsc::Sender<Message>>,
}

impl WebSocketApi {
    /// Create a new WebSocket API instance
    pub fn new(client: &KrakenClient) -> Self {
        Self {
            ws_url: client.config.ws_url.clone(),
            tx: None,
        }
    }
    
    /// Connect to the WebSocket API
    pub async fn connect(&mut self) -> Result<mpsc::Receiver<Result<WebSocketMessage>>> {
        // Create message channels
        let (tx, mut rx) = mpsc::channel::<Message>(100);
        let (message_tx, message_rx) = mpsc::channel::<Result<WebSocketMessage>>(100);
        
        // Store the channel
        self.tx = Some(tx);
        
        // Connect to the WebSocket
        let url = Url::parse(&self.ws_url).map_err(|e| Error::WebSocket(format!("Invalid URL: {}", e)))?;
        let (ws_stream, _) = connect_async(url).await.map_err(|e| Error::WebSocket(format!("Connection error: {}", e)))?;
        let (write, mut read) = ws_stream.split();
        
        // Wrap the write sink in an Arc<Mutex<_>> to share between tasks
        let write = Arc::new(Mutex::new(write));
        let write_clone = write.clone();
        
        // Spawn a task to forward messages from the channel to the WebSocket
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                let mut write_lock = write_clone.lock().await;
                if let Err(e) = write_lock.send(message).await {
                    eprintln!("Error sending message: {}", e);
                    break;
                }
            }
        });
        
        // Spawn a task to forward messages from the WebSocket to the channel
        tokio::spawn(async move {
            while let Some(message) = read.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        let result = match serde_json::from_str::<WebSocketMessage>(&text) {
                            Ok(msg) => Ok(msg),
                            Err(e) => {
                                eprintln!("Error parsing message: {}", e);
                                
                                // Try to parse as array
                                match serde_json::from_str::<Vec<Value>>(&text) {
                                    Ok(array) => Ok(WebSocketMessage::DataArray(array)),
                                    Err(e2) => {
                                        eprintln!("Error parsing message as array: {}", e2);
                                        
                                        // Return as generic value
                                        match serde_json::from_str::<Value>(&text) {
                                            Ok(value) => Ok(WebSocketMessage::Generic(value)),
                                            Err(e3) => Err(Error::WebSocket(format!("Failed to parse message: {}", e3))),
                                        }
                                    }
                                }
                            }
                        };
                        
                        if let Err(e) = message_tx.send(result).await {
                            eprintln!("Error forwarding message to channel: {}", e);
                            break;
                        }
                    }
                    Ok(Message::Binary(data)) => {
                        eprintln!("Received binary message: {} bytes", data.len());
                    }
                    Ok(Message::Ping(data)) => {
                        // Automatically respond with a pong
                        let mut write_lock = write.lock().await;
                        if let Err(e) = write_lock.send(Message::Pong(data)).await {
                            eprintln!("Error sending pong: {}", e);
                            break;
                        }
                    }
                    Ok(Message::Pong(_)) => {
                        // Ignore pong messages
                    }
                    Ok(Message::Frame(frame)) => {
                        eprintln!("Received frame message: {:?}", frame);
                    }
                    Ok(Message::Close(frame)) => {
                        eprintln!("WebSocket closed: {:?}", frame);
                        break;
                    }
                    Err(e) => {
                        eprintln!("WebSocket error: {}", e);
                        break;
                    }
                }
            }
        });
        
        Ok(message_rx)
    }
    
    /// Subscribe to a channel
    pub async fn subscribe(&self, request: WebSocketSubscriptionRequest) -> Result<()> {
        let message = serde_json::to_string(&request).map_err(|e| Error::WebSocket(format!("Failed to serialize subscription request: {}", e)))?;
        
        if let Some(tx) = &self.tx {
            tx.send(Message::Text(message)).await.map_err(|e| Error::WebSocket(format!("Failed to send subscription request: {}", e)))?;
        } else {
            return Err(Error::WebSocket("Not connected to WebSocket".to_string()));
        }
        
        Ok(())
    }
    
    /// Unsubscribe from a channel
    pub async fn unsubscribe(&self, request: WebSocketUnsubscriptionRequest) -> Result<()> {
        let message = serde_json::to_string(&request).map_err(|e| Error::WebSocket(format!("Failed to serialize unsubscription request: {}", e)))?;
        
        if let Some(tx) = &self.tx {
            tx.send(Message::Text(message)).await.map_err(|e| Error::WebSocket(format!("Failed to send unsubscription request: {}", e)))?;
        } else {
            return Err(Error::WebSocket("Not connected to WebSocket".to_string()));
        }
        
        Ok(())
    }
    
    /// Send a ping message
    pub async fn ping(&self) -> Result<()> {
        if let Some(tx) = &self.tx {
            tx.send(Message::Ping(vec![])).await.map_err(|e| Error::WebSocket(format!("Failed to send ping: {}", e)))?;
        } else {
            return Err(Error::WebSocket("Not connected to WebSocket".to_string()));
        }
        
        Ok(())
    }
    
    /// Close the connection
    pub async fn close(&self) -> Result<()> {
        if let Some(tx) = &self.tx {
            tx.send(Message::Close(None)).await.map_err(|e| Error::WebSocket(format!("Failed to close connection: {}", e)))?;
        } else {
            return Err(Error::WebSocket("Not connected to WebSocket".to_string()));
        }
        
        Ok(())
    }
}
