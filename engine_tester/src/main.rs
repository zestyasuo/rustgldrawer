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

	let sp2 = ShaderProgram::new(
		"../src/graphics/shaders/vertex.glsl",
		"../src/graphics/shaders/frag2.glsl",
	);

	let triangle1: [f32; 9] = [
		0.0, 0.5, 0.0,
		0.0, 0.3, 0.0,
		-0.5, 0.0, 0.0,
	];

	let triangle2: [f32; 9] = [
		0.0, -0.3, 0.0,
		0.0, -0.5, 0.0,
		0.5, 0.0, 0.0,
	];

	// let indexes = [0, 1, 3, 1, 2, 3];

	let vao1 = Vao::new();
	vao1.bind();
	let vbo_tr1 = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
	vbo_tr1.bind();
	vbo_tr1.store_f32_data(&triangle1);

	let pos_atrr1 = VertexAttribute::new(
		0,
		3,
		gl::FLOAT,
		gl::FALSE,
		3 * mem::size_of::<GLfloat>() as GLsizei,
		ptr::null(),
	);
	pos_atrr1.enable();

	let vao2 = Vao::new();
	vao2.bind();
	let vbo_tr2 = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
	vbo_tr2.bind();
	vbo_tr2.store_f32_data(&triangle2);

	let pos_attr2 = VertexAttribute::new(
		0,
		3,
		gl::FLOAT,
		gl::FALSE,
		3 * mem::size_of::<GLfloat>() as GLsizei,
		ptr::null(),
	);
	pos_attr2.enable();
	// let ebo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
	// ebo.bind();
	// ebo.store_i32_data(&indexes);
	// let index_attribute = VertexAttribute::new(
	// 	1,
	// 	3,
	// 	gl::FLOAT,
	// 	gl::FALSE,
	// 	3 * mem::size_of::<GLfloat>() as GLsizei,
	// 	ptr::null(),
	// );

	// index_attribute.enable();

	while !win.should_close() {
		unsafe {
			gl::ClearColor(0.2, 0.3, 0.3, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
			sp.bind();
			vao1.bind();
			gl::DrawArrays(gl::TRIANGLES, 0, 3);
			sp2.bind();
			vao2.bind();
			gl::DrawArrays(gl::TRIANGLES, 0, 3);
			// gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
		}
		win.update();
	}
}
