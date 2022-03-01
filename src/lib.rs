use std::ops::Deref;

#[macro_use]
extern crate thiserror;

pub mod error;
pub mod raster;
pub mod render;

pub const SCREEN_HTTP_URL: &'static str = "http://10.1.0.198:8000";
pub const SCREEN_WS_URL: &'static str = "ws://10.1.0.198:8000/set_pixel";

/// Main configuration for the renderer
#[derive(Debug, Clone, Copy)]
pub struct Config {
	/// Width of the display
	pub display_width:  usize,
	/// Height of the display
	pub display_height: usize,
	/// Primitive to target
	pub primitive:      Primitive,
}

impl Config {
	pub fn new(width: usize, height: usize, primitive: Primitive) -> Self {
		Self { display_width: width, display_height: height, primitive }
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Primitive {
	/// Treat each vertex as defining a point, won't draw any lines
	Point,
	/// Treat each separate pair of adjacent vertices as defining a line
	/// eg. vertices 0 and 1 are a line, 2 and 3 are a line, ...
	/// An unmatched vertex at the end will be ignored
	Line,
	/// All pairs of adjacent vertices are considered as defining a line
	/// eg. vertices 0 and 1 are a line, 1 and 2 are a line, ...
	/// If only one vertex is given nothing will be drawn
	LineStrip,
	/// Same as LineStrip, but connects the first and last vertex
	/// Essentially the same as drawing a polygon
	LineLoop,
	/// Same as LineLoop, but fills the resulting polygon with color
	LineLoopFilled,
	/// Treat each separate 3-tuple of vertices as defining a triangle
	/// eg. vertices 0, 1, and 2 are a triangle; 3, 4, and 5 are a triangle, ...
	/// Unmatched vertices are ignored
	Triangle,
	/// All 3-tuples of adjacent vertices define a triangle
	/// eg. vertices 0, 1, and 2 are a triangle; 1, 2, and 3 are a triangle, ...
	/// If less than 3 vertices are given nothing is drawn
	TriangleStrip,
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
	pub x: isize,
	pub y: isize,
}

#[derive(Debug, Clone)]
pub struct VertexBuffer {
	vertices: Vec<Vertex>,
}

// Implement Deref so you can call VertexBuffer.iter()
impl Deref for VertexBuffer {
	type Target = Vec<Vertex>;

	fn deref(&self) -> &Self::Target { &self.vertices }
}

impl From<Vec<Vertex>> for VertexBuffer {
	fn from(v: Vec<Vertex>) -> Self { Self { vertices: v } }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixel {
	/// X coordinate
	pub x:     isize,
	/// Y coordinate
	pub y:     isize,
	/// RGBA color
	pub color: (u8, u8, u8, u8),
}

impl std::fmt::Display for Pixel {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{} {} {} {} {}",
			self.x, self.y, self.color.0, self.color.1, self.color.2
		)
	}
}

#[derive(Debug, Clone)]
pub struct PixelBuffer {
	pixels: Vec<Pixel>,
}

impl Deref for PixelBuffer {
	type Target = Vec<Pixel>;

	fn deref(&self) -> &Self::Target { &self.pixels }
}

impl From<Vec<Pixel>> for PixelBuffer {
	fn from(v: Vec<Pixel>) -> Self { Self { pixels: v } }
}
