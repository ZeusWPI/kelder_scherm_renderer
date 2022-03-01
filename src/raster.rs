//! Rasterization functions for VertexBuffers

use crate::{Config, Pixel, PixelBuffer, Primitive, VertexBuffer};

impl VertexBuffer {
	/// Rasterize the given list of vertices using Bresenham's algorithm
	///
	/// ## Arguments
	/// - cfg: [Config](super::Config) - The renderer configuration
	/// - buff: [VertexBuffer](super::VertexBuffer) - The list of vertices to draw
	pub fn rasterize_scan(&self, cfg: &Config) -> PixelBuffer {
		let mut pixels = Vec::<Pixel>::new();

		match cfg.primitive {
			Primitive::Point => {
				pixels = self
					.iter()
					.map(|v| {
						Pixel {
							x:     v.0,
							y:     v.1,
							color: (255, 255, 255, 255),
						}
					})
					.collect();
			},
			_ => (),
		}

		pixels.into()
	}
}
