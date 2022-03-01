use kelder_scherm_renderer::error::RenderError;
use kelder_scherm_renderer::{Config, Primitive, Vertex, VertexBuffer};

#[tokio::main]
async fn main() -> Result<(), RenderError> {
	let cfg = Config::new(600, 800, Primitive::Triangle);

	let verts = vec![
		Vertex(150, 25),
		Vertex(173, 38),
		Vertex(146, 31),
		Vertex(250, 25),
		Vertex(260, 60),
		Vertex(240, 45),
	];

	let mut vbuf = VertexBuffer::from(verts);
	let pbuf = vbuf.rasterize_scan(&cfg);

	// println!("{}", pbuf);

	pbuf.render_pixels().await?;

	Ok(())
}
