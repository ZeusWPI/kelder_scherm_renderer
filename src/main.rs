use kelder_scherm_renderer::error::RenderError;
use kelder_scherm_renderer::{Config, Primitive, Vertex, VertexBuffer};

#[tokio::main]
async fn main() -> Result<(), RenderError> {
	let cfg = Config::new(600, 800, Primitive::Triangle);

	let verts = vec![
		Vertex(250, 250),
		Vertex(270, 250),
		Vertex(260, 265),
		Vertex(150, 150),
		Vertex(170, 150),
		Vertex(160, 165),
	];

	let vbuf = VertexBuffer::from(verts);
	let pbuf = vbuf.rasterize_scan(&cfg);

	println!("{}", pbuf);

	pbuf.render_pixels().await?;

	Ok(())
}
