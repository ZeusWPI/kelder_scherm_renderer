use kelder_scherm_renderer::error::RenderError;
use kelder_scherm_renderer::{VertexBuffer, Config, Primitive, Vertex};

#[tokio::main]
async fn main() -> Result<(), RenderError> {
	let cfg = Config::new(600, 800, Primitive::Line);

	let verts = vec![
		Vertex { x: 100, y: 100 },
		Vertex { x: 90, y: 90 },
	];

	let vbuf = VertexBuffer::from(verts);
	let pbuf = vbuf.rasterize_scan(&cfg);

	pbuf.render_pixels().await?;

	Ok(())
}
