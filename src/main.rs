use kelder_scherm_renderer::error::RenderError;
use kelder_scherm_renderer::{Config, Primitive, Vertex, VertexBuffer};

#[tokio::main]
async fn main() -> Result<(), RenderError> {
	let cfg = Config::new(600, 800, Primitive::TriangleWireStrip);

	let verts = vec![Vertex(140, 290), Vertex(150, 290), Vertex(145, 280), Vertex(155, 280)];

	let mut vbuf = VertexBuffer::from(verts);

	let pbuf = vbuf.rasterize_scan(&cfg);

	// println!("{}", pbuf);

	pbuf.render_pixels().await?;

	Ok(())
}
