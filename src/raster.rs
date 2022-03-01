//! Rasterization functions for VertexBuffers

use std::iter::zip;
use std::sync::{Arc, Mutex};
use std::thread;

use rayon::prelude::*;

use crate::{Config, Pixel, PixelBuffer, Primitive, Vertex, VertexBuffer};

impl VertexBuffer {
	/// Rasterize the given list of vertices as the requested primitive
	///
	/// ## Arguments
	/// - cfg: [Config](super::Config) - The renderer configuration
	/// - buff: [VertexBuffer](super::VertexBuffer) - The list of vertices to draw
	pub fn rasterize_scan(&mut self, cfg: &Config) -> PixelBuffer {
		let p_vec = match cfg.primitive {
			Primitive::Point => rasterize_points(self),
			Primitive::Line => rasterize_lines(self),
			Primitive::LineStrip => rasterize_line_strips(self),
			Primitive::LineLoop => rasterize_line_loops(self),
			Primitive::Triangle => rasterize_triangles(self),
			Primitive::TriangleStrip => rasterize_triangle_strips(self),
			Primitive::TriangleWire => rasterize_triangle_wires(self),
			Primitive::TriangleWireStrip => rasterize_triangle_wire_strips(self),
		};

		p_vec.par_iter()
			.filter(|p| {
				(p.x as usize) < cfg.display_width
					&& (p.y as usize) < cfg.display_height
			})
			.map(|p| *p)
			.collect::<Vec<Pixel>>()
			.into()
	}
}

#[inline(always)]
fn rasterize_points(vbuf: &[Vertex]) -> Vec<Pixel> {
	vbuf.par_iter()
		.map(|vertex| {
			Pixel { x: vertex.0, y: vertex.1, color: (255, 255, 255, 255) }
		})
		.collect()
}

#[inline(always)]
fn rasterize_lines(vbuf: &[Vertex]) -> Vec<Pixel> {
	vbuf.par_chunks_exact(2)
		.map(|pair| bresenham_rasterize(pair))
		.collect::<Vec<Vec<Pixel>>>()
		.concat()
}

#[inline(always)]
fn rasterize_line_strips(vbuf: &[Vertex]) -> Vec<Pixel> {
	let v_stripbuf = VertexLineStripBuffer { buf: vbuf, idx: 0 };

	v_stripbuf
		.par_bridge()
		.map(|pair| bresenham_rasterize(&pair))
		.collect::<Vec<Vec<Pixel>>>()
		.concat()
}

#[inline(always)]
fn rasterize_line_loops(vbuf: &[Vertex]) -> Vec<Pixel> {
	let mut looped_buffer = vbuf.to_vec();
	looped_buffer.push(vbuf[0]);

	let v_loopbuf = VertexLineStripBuffer { buf: &looped_buffer, idx: 0 };

	v_loopbuf
		.par_bridge()
		.map(|pair| bresenham_rasterize(&pair))
		.collect::<Vec<Vec<Pixel>>>()
		.concat()
}

#[inline(always)]
fn rasterize_triangle_wires(vbuf: &[Vertex]) -> Vec<Pixel> {
	vbuf.par_chunks_exact(3)
		.map(|triplet| bresenham_rasterize_triangle(triplet))
		.collect::<Vec<Vec<Pixel>>>()
		.concat()
}

#[inline(always)]
fn rasterize_triangle_wire_strips(vbuf: &[Vertex]) -> Vec<Pixel> {
	let v_stripbuf = VertexTriangleStripBuffer { buf: vbuf, idx: 1 };

	v_stripbuf
		.par_bridge()
		.map(|triplet| bresenham_rasterize_triangle(&triplet))
		.collect::<Vec<Vec<Pixel>>>()
		.concat()
}

#[inline(always)]
fn rasterize_triangles(vbuf: &mut [Vertex]) -> Vec<Pixel> {
	vbuf.par_chunks_exact_mut(3)
		.map(|triplet| rasterize_triangle(triplet))
		.collect::<Vec<Vec<Pixel>>>()
		.concat()
}

#[inline(always)]
fn rasterize_triangle_strips(vbuf: &mut [Vertex]) -> Vec<Pixel> {
	let v_stripbuf = VertexTriangleStripBuffer { buf: vbuf, idx: 1 };

	v_stripbuf
		.par_bridge()
		.map(|mut triplet| rasterize_triangle(&mut triplet))
		.collect::<Vec<Vec<Pixel>>>()
		.concat()
}

struct VertexLineStripBuffer<'a> {
	buf: &'a [Vertex],
	idx: usize,
}

impl<'a> Iterator for VertexLineStripBuffer<'a> {
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

struct VertexTriangleStripBuffer<'a> {
	buf: &'a [Vertex],
	idx: usize,
}

impl<'a> Iterator for VertexTriangleStripBuffer<'a> {
	type Item = Vec<Vertex>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.idx < self.buf.len() - 1 {
			self.idx += 1;
			Some(vec![
				self.buf[self.idx - 2],
				self.buf[self.idx - 1],
				self.buf[self.idx],
			])
		} else {
			None
		}
	}
}

fn rasterize_triangle(vbuf: &mut [Vertex]) -> Vec<Pixel> {
	vbuf.sort_by(|a, b| a.1.cmp(&b.1));

	// v0.y <= v1.y <= v2.y
	// Flat bottom triangle
	if vbuf[1].1 == vbuf[2].1 {
		bresenham_rasterize_fill_flat(&vbuf)
	// Flat top triangle
	} else if vbuf[0].1 == vbuf[1].1 {
		bresenham_rasterize_fill_flat(&vec![vbuf[2], vbuf[1], vbuf[0]])
	} else {
		let x4 = ((vbuf[0].0 as f64)
			+ (((vbuf[1].1 - vbuf[0].1) as f64) / ((vbuf[2].1 - vbuf[0].1) as f64))
				* ((vbuf[2].0 - vbuf[0].0) as f64)) as isize;

		let split_vertex = Vertex(x4, vbuf[1].1);
		let mut pbuf = bresenham_rasterize_fill_flat(&vec![vbuf[0], vbuf[1], split_vertex]);
		pbuf.append(&mut bresenham_rasterize_fill_flat(&vec![
			vbuf[2],
			vbuf[1],
			split_vertex,
		]));

		pbuf
	}
}

/// First vertex is assumed to have smallest (closest to top of screen) y coordinate
fn bresenham_rasterize_fill_flat(triplet: &[Vertex]) -> Vec<Pixel> {
	let Vertex(mut x0, mut y0) = triplet[0];
	let Vertex(x1, y1) = triplet[1];
	let Vertex(x2, y2) = triplet[2];

	let dx01 = (x1).abs_diff(x0) as isize;
	let dx02 = (x2).abs_diff(x0) as isize;

	let sx01 = if x0 < x1 { 1 } else { -1 };
	let sx02 = if x0 < x2 { 1 } else { -1 };

	let dy01 = -((y1).abs_diff(y0) as isize);
	let dy02 = -((y2).abs_diff(y0) as isize);

	let sy01 = if y0 < y1 { 1 } else { -1 };
	let sy02 = if y0 < y2 { 1 } else { -1 };

	let mut error01 = dx01 + dy01;
	let mut error02 = dx02 + dy02;

	let mut pixel_vec = Vec::<Pixel>::new();

	// Points at which v01 moved one pixel in the y direction
	let v01_line_endpoints = Arc::new(Mutex::new(Vec::<(isize, isize)>::new()));
	let v01_line_endpoints_clone = Arc::clone(&v01_line_endpoints);
	// Points at which v02 moved one pixel in the y direction
	let mut v02_line_endpoints = Vec::<(isize, isize)>::new();

	// v0 - v1 loop
	let v01_thread = thread::spawn(move || {
		loop {
			if (x0 == x1) && (y0 == y1) {
				break;
			}

			let e2 = error01 * 2;
			if e2 >= dy01 {
				if x0 == x1 {
					break;
				}

				error01 += dy01;
				x0 += sx01;
			}
			if e2 <= dx01 {
				if y0 == y1 {
					break;
				}

				error01 += dx01;
				y0 += sy01;
				v01_line_endpoints_clone.lock().unwrap().push((x0, y0))
			}
		}
	});

	Vertex(x0, y0) = triplet[0];

	// v0 - v2 loop
	loop {
		if (x0 == x1) && (y0 == y1) {
			break;
		}

		let e2 = error02 * 2;
		if e2 >= dy02 {
			if x0 == x1 {
				break;
			}

			error02 += dy02;
			x0 += sx02;
		}
		if e2 <= dx02 {
			if y0 == y1 {
				break;
			}

			error02 += dx02;
			y0 += sy02;
			v02_line_endpoints.push((x0, y0))
		}
	}

	pixel_vec.push(Pixel {
		x:     triplet[0].0,
		y:     triplet[0].1,
		color: (255, 255, 255, 255),
	});

	v01_thread.join().unwrap();

	let line_points = zip(v01_line_endpoints.lock().unwrap().to_owned(), v02_line_endpoints);

	for (start, end) in line_points {
		let y = start.1;
		for x in start.0.min(end.0)..=start.0.max(end.0) {
			pixel_vec.push(Pixel { x, y, color: (255, 255, 255, 255) });
		}
	}

	pixel_vec
}

fn bresenham_rasterize_triangle(triplet: &[Vertex]) -> Vec<Pixel> {
	let mut pbuf = bresenham_rasterize(&triplet[0..=1]);
	pbuf.append(&mut bresenham_rasterize(&triplet[1..=2]));
	pbuf.append(&mut bresenham_rasterize(&vec![triplet[2], triplet[0]]));

	pbuf
}

fn bresenham_rasterize(pair: &[Vertex]) -> Vec<Pixel> {
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
