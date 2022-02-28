use kelder_scherm_renderer::error::RenderError;
use kelder_scherm_renderer::{Pixel, PixelBuffer};

#[tokio::main]
async fn main() -> Result<(), RenderError> {
	let pixels = vec![
		Pixel { x: 100, y: 100, color: (255, 255, 255, 255) },
		Pixel { x: 100, y: 101, color: (255, 255, 255, 255) },
		Pixel { x: 100, y: 102, color: (255, 255, 255, 255) },
		Pixel { x: 100, y: 103, color: (255, 255, 255, 255) },
		Pixel { x: 100, y: 104, color: (255, 255, 255, 255) },
		Pixel { x: 100, y: 105, color: (255, 255, 255, 255) },
	];
	let buf = PixelBuffer::from(pixels);

	buf.render_pixels().await?;

	Ok(())
}
