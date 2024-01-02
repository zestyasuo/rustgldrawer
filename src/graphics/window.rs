use glfw::*;
// use std::sync::mpsc::Receiver;

pub struct Window {
	glfw: glfw::Glfw,
	window_handle: glfw::PWindow,
	events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
	pub fn new(width: u32, height: u32, title: &str) -> Window {
		let mut glfw = glfw::init(fail_on_errors!()).unwrap();

		let (mut window, events) = glfw
			.create_window(width, height, title, glfw::WindowMode::Windowed)
			.expect("Failed");

		glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
		glfw.window_hint(glfw::WindowHint::OpenGlProfile(
			glfw::OpenGlProfileHint::Core,
		));
		window.make_current();
		window.set_framebuffer_size_polling(true);
		window.set_key_polling(true);
		Window {
			glfw,
			window_handle: window,
			events,
		}
	}

	pub fn init_gl(&mut self) {
		self.window_handle.make_current();
		gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
	}

	pub fn should_close(&self) -> bool {
		return self.window_handle.should_close();
	}

	pub fn update(&mut self) {
		self.process_events();
		self.glfw.poll_events();
		self.window_handle.swap_buffers();
	}

	fn process_events(&mut self) {
		for (_, event) in glfw::flush_messages(&self.events) {
			match event {
				glfw::WindowEvent::FramebufferSize(width, height) => {
					// Make sure the viewport matches the new window dimensions.
					unsafe { gl::Viewport(0, 0, width, height) }
				}
				glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
					self.window_handle.set_should_close(true)
				}
				_ => {}
			}
		}
	}
}
