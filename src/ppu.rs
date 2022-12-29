use minifb::*;

const WIDTH: usize = 256;
const HEIGHT: usize = 240;

pub struct PPU {
	window: Window
}

impl PPU {
	pub fn new() -> Self {
		let options = WindowOptions {
		        scale: Scale::X2,
		        ..WindowOptions::default()
		    };
	    let mut window = Window::new(
		        "Blit Example - ESC to exit & click to draw",
		        HEIGHT,
		        WIDTH,
		        options,
		    )
		    .expect("Unable to open window");

		Self {
			window: window
		}
	}

	pub fn update(self: &mut Self) {
		let mut buffer: Vec<u32> = vec![0x00_FF_FF_FF; WIDTH * HEIGHT];

		self.window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
	}
		
}