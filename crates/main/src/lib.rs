use {
	wasm_bindgen::prelude::*,
	winit::{
		event_loop::*,
		window::*,
		event::*,
		dpi::PhysicalSize,
		platform::web::WindowExtWebSys,
	},
};


struct State {
	surface: wgpu::Surface,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	size: PhysicalSize<u32>,
	window: Window,
}

impl State {
	async fn new(window: Window) -> Self {
		let size = window.inner_size();

		let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
			backends: wgpu::Backends::all(),
			dx12_shader_compiler: Default::default(),
		});

		// # Safety
		// The surface needs to live as long as the window that created it.
		// State owns the window so this should be safe.
		let surface = unsafe { instance.create_surface(&window) }.expect("Failed to create surface");

		let adapter = instance.request_adapter(
			&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::default(),
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
			},
		).await.expect("Failed to request adapter");

		let (device, queue) = adapter.request_device(
			&wgpu::DeviceDescriptor {
				features: wgpu::Features::empty(),
				limits: wgpu::Limits::downlevel_webgl2_defaults(),
				label: None,
			},
			None,
		).await.expect("Failed to request device");

		let surface_caps = surface.get_capabilities(&adapter);
		// Shader code in this tutorial assumes an sRGB surface texture. Using a different
		// one will result all the colors coming out darker. If you want to support non
		// sRGB surfaces, you'll need to account for that when drawing to the frame.
		let surface_format = surface_caps.formats.iter()
			.copied()
			.find(|f| f.is_srgb())
			.unwrap_or(surface_caps.formats[0]);
		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface_format,
			width: size.width,
			height: size.height,
			present_mode: surface_caps.present_modes[0],
			alpha_mode: surface_caps.alpha_modes[0],
			view_formats: vec![],
		};
		surface.configure(&device, &config);

		Self {
			window,
			surface,
			device,
			queue,
			config,
			size,
		}
	}

	pub fn window(&self) -> &Window {
		&self.window
	}

	fn resize(&mut self, new_size: PhysicalSize<u32>) {
		if new_size.width > 0 && new_size.height > 0 {
			self.size = new_size;
			self.config.width = new_size.width;
			self.config.height = new_size.height;
			self.surface.configure(&self.device, &self.config);
		}
	}

	fn input(&mut self, event: &WindowEvent) -> bool {
		false
	}

	fn update(&mut self) {

	}

	fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
		let output = self.surface.get_current_texture()?;
		let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
		let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("Render Encoder"),
		});

		{
			let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("Render Pass"),
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: &view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(wgpu::Color {
							r: 0.1,
							g: 0.2,
							b: 0.3,
							a: 1.0,
						}),
						store: true,
					},
				})],
				depth_stencil_attachment: None,
			});
		}

		// submit will accept anything that implements IntoIter
		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();

		Ok(())
	}
}

#[wasm_bindgen]
pub async fn run(id: String) {
	std::panic::set_hook(Box::new(console_error_panic_hook::hook));
	console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");

	let event_loop = EventLoop::new();
	let window = WindowBuilder::new().build(&event_loop).unwrap();

	window.set_inner_size(PhysicalSize::new(512, 512));

	let document = web_sys::window()
		.and_then(|win| win.document()).expect("Failed to get document");

	let target_element = document.get_element_by_id(&id).expect("Target element is not exists");

	let canvas = web_sys::Element::from(window.canvas());

	target_element.append_child(&canvas).expect("Failed to bind canvas");


	let mut state = State::new(window).await;

	event_loop.run(
		move |event, _, control_flow| match event {
			Event::WindowEvent {
				ref event,
				window_id,
			} if window_id == state.window.id() => match event {
				WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
					input: KeyboardInput {
						state: ElementState::Pressed,
						virtual_keycode: Some(VirtualKeyCode::Escape),
						..
					},
					..
				} => *control_flow = ControlFlow::Exit,
				_ => {}
			}
			Event::RedrawRequested(window_id) if window_id == state.window().id() => {
				state.update();
				match state.render() {
					Ok(_) => {}
					// Reconfigure the surface if lost
					Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
					// The system is out of memory, we should probably quit
					Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
					// All other errors (Outdated, Timeout) should be resolved by the next frame
					Err(e) => eprintln!("{:?}", e),
				}
			}
			Event::MainEventsCleared => {
				// RedrawRequested will only trigger once, unless we manually
				// request it.
				state.window().request_redraw();
			}
			_ => {}
		}
	);
}
