//! Defines a common error type

use tokio_tungstenite::tungstenite;

#[derive(Debug, Error)]
pub enum RenderError {
	#[error("Websocket Error: {0:?}")]
	WebsocketError(#[from] tungstenite::Error),
}
