use kelder_scherm_renderer::error::RenderError;
use kelder_scherm_renderer::{Config, Primitive, Vertex, VertexBuffer};

#[tokio::main]
async fn main() -> Result<(), RenderError> {
	let cfg = Config::new(600, 800, Primitive::TriangleStrip);

	let left_side = vec![
		Vertex(44, 59),
		Vertex(56, 59),
		Vertex(50, 50),
		Vertex(61, 50),
		Vertex(56, 44),
	];

	let right_side = vec![
		Vertex(44, 59),
		Vertex(50, 50),
		Vertex(39, 50),
		Vertex(44, 44),
		Vertex(56, 44),
	];

	let top = vec![
		Vertex(44, 44),
		Vertex(56, 44),
		Vertex(50, 53),
	];

	let mut buf_l = VertexBuffer::from(left_side);
	let mut buf_r = VertexBuffer::from(right_side);
	let mut buf_t = VertexBuffer::from(top);

	let pbuf_l = buf_l.rasterize_scan(&cfg);
	let pbuf_r = buf_r.rasterize_scan(&cfg);
	let pbuf_t = buf_t.rasterize_scan(&cfg);

	// println!("{}", pbuf);

	pbuf_l.render_pixels().await?;
	pbuf_r.render_pixels().await?;
	pbuf_t.render_pixels().await?;

	Ok(())
}
