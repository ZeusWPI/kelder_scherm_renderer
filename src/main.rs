use kelder_scherm_renderer::error::RenderError;
use kelder_scherm_renderer::{Config, Primitive, Vertex, VertexBuffer};

#[tokio::main]
async fn main() -> Result<(), RenderError> {
	let cfg = Config::new(600, 800, Primitive::TriangleStrip);

	let verts = vec![Vertex(240, 290), Vertex(250, 290), Vertex(245, 280), Vertex(255, 280)];

	let mut vbuf = VertexBuffer::from(verts);

	let pbuf = vbuf.rasterize_scan(&cfg);

	// println!("{}", pbuf);

	pbuf.render_pixels().await?;

	Ok(())
}
