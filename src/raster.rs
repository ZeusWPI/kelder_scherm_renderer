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
			Primitive::Point => rasterize_point(self).into(),
			Primitive::Line => rasterize_line(self).into(),
			Primitive::LineStrip => rasterize_line_strip(self).into(),
			Primitive::LineLoop => rasterize_line_loop(self).into(),
			Primitive::Triangle => rasterize_triangle(self).into(),
			Primitive::TriangleStrip => Vec::new().into(),
			Primitive::TriangleWire => Vec::new().into(),
			Primitive::TriangleWireStrip => Vec::new().into(),
		}
	}
}

#[inline(always)]
fn rasterize_point(vbuf: &[Vertex]) -> Vec<Pixel> {
	vbuf.par_iter()
		.map(|vertex| {
			Pixel { x: vertex.0, y: vertex.1, color: (255, 255, 255, 255) }
		})
		.collect()
}

#[inline(always)]
fn rasterize_line(vbuf: &[Vertex]) -> Vec<Pixel> {
	vbuf.par_chunks_exact(2)
		.map(|pair| bresenham_scan(pair))
		.collect::<Vec<Vec<Pixel>>>()
		.concat()
}

#[inline(always)]
fn rasterize_line_strip(vbuf: &[Vertex]) -> Vec<Pixel> {
	let v_stripbuf = VertexStripBuffer { buf: vbuf, idx: 0 };

	v_stripbuf
		.par_bridge()
		.map(|pair| bresenham_scan(&pair))
		.collect::<Vec<Vec<Pixel>>>()
		.concat()
}

#[inline(always)]
fn rasterize_line_loop(vbuf: &[Vertex]) -> Vec<Pixel> {
	let mut looped_buffer = vbuf.to_vec();
	looped_buffer.push(vbuf[0]);

	let v_loopbuf = VertexStripBuffer { buf: &looped_buffer, idx: 0 };

	v_loopbuf
		.par_bridge()
		.map(|pair| bresenham_scan(&pair))
		.collect::<Vec<Vec<Pixel>>>()
		.concat()
}

#[inline(always)]
fn rasterize_triangle(vbuf: &[Vertex]) -> Vec<Pixel> {
	vbuf.par_chunks_exact(3)
		.map(|triplet| rasterize_line_loop(triplet))
		.collect::<Vec<Vec<Pixel>>>()
		.concat()
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
