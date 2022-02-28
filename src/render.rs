//! Functions to render a PixelBuffer to the screen

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use crate::error::RenderError;
use crate::{PixelBuffer, SCREEN_WS_URL};

impl PixelBuffer {
	/// Render the PixelBuffer to the screen using websockets
	pub async fn render_pixels(&self) -> Result<(), RenderError> {
		let (ws_stream, _) = connect_async(SCREEN_WS_URL).await?;

		let (mut tx, _rx) = ws_stream.split();

		for pixel in self.iter() {
			tx.feed(Message::Text(pixel.to_string())).await?;
		}

		tx.flush().await?;

		Ok(())
	}
}
