use crate::errors::{Error, Result};
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};
use tokio::net::TcpStream;
use url::Url;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::de::DeserializeOwned;
use std::future::Future;
use std::pin::Pin;

/// A generic asynchronous WebSocket client.
///
/// E: The type of event deserialized from messages.
/// H: The type of the handler function.
pub struct AsyncWebsocketClient<'a, E, H>
where
    E: DeserializeOwned + Send + std::fmt::Debug + 'a, // Added Debug
    H: FnMut(E) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> + Send + Sync + 'a,
{
    socket: Arc<Mutex<Option<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    handler: Arc<Mutex<H>>,
    phantom: std::marker::PhantomData<&'a E>,
}

impl<'a, E, H> AsyncWebsocketClient<'a, E, H>
where
    E: DeserializeOwned + Send + std::fmt::Debug + 'a, // Added Debug
    H: FnMut(E) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> + Send + Sync + 'a,
{
    pub fn new(handler: H) -> Self {
        AsyncWebsocketClient {
            socket: Arc::new(Mutex::new(None)),
            handler: Arc::new(Mutex::new(handler)),
            phantom: std::marker::PhantomData,
        }
    }

    pub async fn connect(&self, wss_url: &str) -> Result<()> {
        let url_obj = Url::parse(wss_url).map_err(Error::UrlParser)?;
        let (ws_stream, _response) = connect_async(url_obj.as_str()) // Convert Url to &str
            .await
            .map_err(Error::WebSocket)?;

        let mut socket_guard = self.socket.lock().await;
        *socket_guard = Some(ws_stream);
        Ok(())
    }

    pub async fn disconnect(&self) -> Result<()> {
        let mut socket_guard = self.socket.lock().await;
        if let Some(stream) = socket_guard.as_mut() {
            stream.close(None).await.map_err(Error::WebSocket)?;
            *socket_guard = None;
            Ok(())
        } else {
            Err(Error::Custom("Not connected".to_string()))
        }
    }

    async fn handle_message_text(&self, msg_text: String) -> Result<()> {
        // This parsing logic might need to be customized based on how
        // Binance wraps multi-stream data or other specific message formats.
        // For now, assuming direct deserialization or a simple 'data' field check.

        // Attempt direct deserialization
        if let Ok(event) = serde_json::from_str::<E>(&msg_text) {
            let mut handler_guard = self.handler.lock().await;
            (handler_guard)(event).await?;
            Ok(())
        } else {
            // If direct deserialization fails, check for a common "data" wrapper
            // This is a simplified example; real-world scenarios might be more complex.
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&msg_text) {
                if let Some(data_val) = value.get("data") {
                    match serde_json::from_value::<E>(data_val.clone()) {
                        Ok(event) => {
                            let mut handler_guard = self.handler.lock().await;
                            (handler_guard)(event).await?;
                            return Ok(());
                        }
                        Err(e_inner) => {
                            return Err(Error::Json(e_inner));
                        }
                    }
                }
                // If not a "data" wrapper, or if that also fails to parse as E
                if let Ok(stream_value) = serde_json::from_str::<serde_json::Value>(&msg_text) {
                    if let Some(_stream_name) = stream_value.get("stream") {
                        if let Some(data_val_stream) = stream_value.get("data") {
                            match serde_json::from_value::<E>(data_val_stream.clone()) {
                                Ok(event) => {
                                    let mut handler_guard = self.handler.lock().await;
                                    (handler_guard)(event).await?;
                                    return Ok(());
                                }
                                Err(e_inner_stream) => {
                                    return Err(Error::Json(e_inner_stream));
                                }
                            }
                        }
                    }
                }
            }
            // If all attempts fail, return original direct deserialization error
            Err(Error::Json(
                serde_json::from_str::<E>(&msg_text).unwrap_err(),
            ))
        }
    }

    pub async fn event_loop(&self, running: Arc<std::sync::atomic::AtomicBool>) -> Result<()> {
        while running.load(std::sync::atomic::Ordering::Relaxed) {
            let mut socket_guard = self.socket.lock().await;
            if let Some(stream) = socket_guard.as_mut() {
                match stream.next().await {
                    Some(Ok(message)) => {
                        drop(socket_guard); // Release lock before handling message
                        match message {
                            Message::Text(text) => {
                                if let Err(e) = self.handle_message_text(text).await {
                                    // Log error or propagate? For now, let's propagate critical parsing/handling errors.
                                    // Specific errors like pings being unhandled by user might be logged and continued.
                                    eprintln!("Error handling message: {:?}", e); // Temporary logging
                                    // Depending on severity, may want to break or continue.
                                    // For now, if handle_message_text returns an error, we propagate it.
                                    return Err(e);
                                }
                            }
                            Message::Binary(_) => { /* Handle binary data if necessary */ }
                            Message::Ping(payload) => {
                                // Re-acquire lock to send Pong
                                let mut new_socket_guard = self.socket.lock().await;
                                if let Some(s) = new_socket_guard.as_mut() {
                                    s.send(Message::Pong(payload))
                                        .await
                                        .map_err(Error::WebSocket)?;
                                }
                                drop(new_socket_guard);
                            }
                            Message::Pong(_) => { /* Pong received */ }
                            Message::Close(close_frame) => {
                                eprintln!("WebSocket closed by server: {:?}", close_frame);
                                return Err(Error::Custom(format!(
                                    "WebSocket closed by server: {:?}",
                                    close_frame
                                )));
                            }
                            Message::Frame(_) => { /* Low-level frame, usually not handled directly */
                            }
                        }
                    }
                    Some(Err(e)) => {
                        // WebSocket stream error
                        return Err(Error::WebSocket(e));
                    }
                    None => {
                        // Stream ended (disconnected)
                        return Err(Error::Custom("WebSocket stream ended".to_string()));
                    }
                }
            } else {
                // Socket not connected, maybe wait and retry or break
                drop(socket_guard);
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
        Ok(())
    }
}
