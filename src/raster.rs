//! Rasterization functions for VertexBuffers

use rayon::prelude::*;

use crate::{Config, Pixel, PixelBuffer, Primitive, VertexBuffer, Vertex};

impl VertexBuffer {
	/// Rasterize the given list of vertices using Bresenham's algorithm
	///
	/// ## Arguments
	/// - cfg: [Config](super::Config) - The renderer configuration
	/// - buff: [VertexBuffer](super::VertexBuffer) - The list of vertices to draw
	pub fn rasterize_scan(&self, cfg: &Config) -> PixelBuffer {
		match cfg.primitive {
			Primitive::Point => {
				self
					.iter()
					.map(|v| {
						Pixel {
							x:     v.x,
							y:     v.y,
							color: (255, 255, 255, 255),
						}
					})
					.collect::<Vec<Pixel>>()
					.into()
			},
			Primitive::Line => rasterize_line(self),
			_ => PixelBuffer { pixels: vec![] },
		}
	}
}

fn rasterize_line(vbuf: &[Vertex]) -> PixelBuffer {
	let pbuf = vbuf
		.par_chunks_exact(2)
		.collect::<Vec<&[Vertex]>>()
		.par_iter()
		.map(|pair| bresenham_scan(pair))
		.collect::<Vec<Vec<Pixel>>>()
		.concat();

	pbuf.into()
}

fn bresenham_scan(pair: &[Vertex]) -> Vec<Pixel> {
	let Vertex { x: mut x0, y: mut y0 } = pair[0];
	let Vertex { x: x1, y: y1 } = pair[1];

	let dx = (x1).abs_diff(x0) as isize;
	let sx = if x0 < x1 { 1 } else { -1 };
	let dy = -((y1).abs_diff(y0) as isize);
	let sy = if y0 < y1 { 1 } else { -1 };
	let mut error = dx + dy;

	let mut pixel_vec = Vec::<Pixel>::new();

	loop {
		pixel_vec.push(Pixel { x: x0, y: y0, color: (255, 255, 255, 255) });

		if (x0 == x1) && (y0 == y1) { break }

		let e2 = error * 2;
		if e2 >= dy {
			if x0 == x1 { break }

			error += dy;
			x0 += sx;
		}
		if e2 <= dx {
			if y0 == y1 { break }

			error += dx;
			y0 += sy;
		}
	}

	pixel_vec
}
