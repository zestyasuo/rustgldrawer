use std::{mem, ptr};
use gl::types::*;

use rustgldrawer::graphics::{window::Window, glwrapper::*};
fn main() {
	let mut win = Window::new(1080, 720, "gl");
	win.init_gl();

	let sp = ShaderProgram::new(
		"../src/graphics/shaders/vertex.glsl",
		"../src/graphics/shaders/frag.glsl",
	);

	let vertices: [f32; 12] = [
		0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0,
	];

	let indexes = [0, 1, 3, 1, 2, 3];

	let vao = Vao::new();
	vao.bind();

	let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
	vbo.bind();
	vbo.store_f32_data(&vertices);

	let ebo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
	ebo.bind();
	ebo.store_i32_data(&indexes);
	let position_attribute = VertexAttribute::new(
		0,
		3,
		gl::FLOAT,
		gl::FALSE,
		3 * mem::size_of::<GLfloat>() as GLsizei,
		ptr::null(),
	);

	let index_attribute = VertexAttribute::new(
		1,
		3,
		gl::FLOAT,
		gl::FALSE,
		3 * mem::size_of::<GLfloat>() as GLsizei,
		ptr::null(),
	);

	position_attribute.enable();
	index_attribute.enable();

	while !win.should_close() {
		unsafe {
			gl::ClearColor(0.2, 0.3, 0.3, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
			sp.bind();
			vao.bind();
			gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
		}
		win.update();
	}
}
