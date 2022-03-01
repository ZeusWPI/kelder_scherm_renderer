use kelder_scherm_renderer::error::RenderError;
use kelder_scherm_renderer::{Config, Primitive, Vertex, VertexBuffer};

#[tokio::main]
async fn main() -> Result<(), RenderError> {
	let cfg = Config::new(600, 800, Primitive::Triangle);

	let verts = vec![
		Vertex(200, 200),
		Vertex(220, 200),
		Vertex(210, 215),
		Vertex(100, 100),
		Vertex(120, 100),
		Vertex(110, 115),
	];

	let vbuf = VertexBuffer::from(verts);
	let pbuf = vbuf.rasterize_scan(&cfg);

	println!("{}", pbuf);

	pbuf.render_pixels().await?;

	Ok(())
}
