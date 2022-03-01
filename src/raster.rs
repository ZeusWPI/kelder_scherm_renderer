//! Rasterization functions for VertexBuffers

use rayon::prelude::*;

use crate::{Config, Pixel, PixelBuffer, Primitive, Vertex, VertexBuffer};

impl VertexBuffer {
	/// Rasterize the given list of vertices using Bresenham's algorithm
	///
	/// ## Arguments
	/// - cfg: [Config](super::Config) - The renderer configuration
	/// - buff: [VertexBuffer](super::VertexBuffer) - The list of vertices to draw
	pub fn rasterize_scan(&self, cfg: &Config) -> PixelBuffer {
		match cfg.primitive {
			Primitive::Point => {
				self.iter()
					.map(|v| {
						Pixel {
							x:     v.0,
							y:     v.1,
							color: (255, 255, 255, 255),
						}
					})
					.collect::<Vec<Pixel>>()
					.into()
			},
			Primitive::Line => rasterize_line(self),
			Primitive::LineStrip => rasterize_line_strip(self),
			Primitive::LineLoop => rasterize_line_loop(self),
			_ => PixelBuffer { pixels: vec![] },
		}
	}
}

fn rasterize_line(vbuf: &[Vertex]) -> PixelBuffer {
	vbuf.par_chunks_exact(2)
		.collect::<Vec<&[Vertex]>>()
		.par_iter()
		.map(|pair| bresenham_scan(pair))
		.collect::<Vec<Vec<Pixel>>>()
		.concat()
		.into()
}

fn rasterize_line_strip(vbuf: &[Vertex]) -> PixelBuffer {
	let v_stripbuf = VertexStripBuffer { buf: vbuf, idx: 0 };

	v_stripbuf
		.par_bridge()
		.map(|pair| bresenham_scan(&pair))
		.collect::<Vec<Vec<Pixel>>>()
		.concat()
		.into()
}

fn rasterize_line_loop(vbuf: &[Vertex]) -> PixelBuffer {
	let mut looped_buffer = vbuf.to_vec();
	looped_buffer.push(vbuf[0]);

	let v_loopbuf = VertexStripBuffer {
		buf: &looped_buffer,
		idx: 0
	};

	v_loopbuf
		.par_bridge()
		.map(|pair| bresenham_scan(&pair))
		.collect::<Vec<Vec<Pixel>>>()
		.concat()
		.into()
}

struct VertexStripBuffer<'a> {
	buf: &'a [Vertex],
	idx: usize,
}

impl<'a> Iterator for VertexStripBuffer<'a> {
	type Item = Vec<Vertex>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.idx < self.buf.len() - 1 {
			self.idx += 1;
			Some(vec![self.buf[self.idx - 1], self.buf[self.idx]])
		} else {
			None
		}
	}
}

fn bresenham_scan(pair: &[Vertex]) -> Vec<Pixel> {
	let Vertex(mut x0, mut y0) = pair[0];
	let Vertex(x1, y1) = pair[1];

	let dx = (x1).abs_diff(x0) as isize;
	let sx = if x0 < x1 { 1 } else { -1 };
	let dy = -((y1).abs_diff(y0) as isize);
	let sy = if y0 < y1 { 1 } else { -1 };
	let mut error = dx + dy;

	let mut pixel_vec = Vec::<Pixel>::new();

	loop {
		pixel_vec.push(Pixel { x: x0, y: y0, color: (255, 255, 255, 255) });

		if (x0 == x1) && (y0 == y1) {
			break;
		}

		let e2 = error * 2;
		if e2 >= dy {
			if x0 == x1 {
				break;
			}

			error += dy;
			x0 += sx;
		}
		if e2 <= dx {
			if y0 == y1 {
				break;
			}

			error += dx;
			y0 += sy;
		}
	}

	pixel_vec
}
