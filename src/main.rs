extern crate sfml;

use sfml::graphics::*;
use sfml::system::Vector2f;
use sfml::window::*;

const WIN_SIZE: (u32, u32) = (800, 600);
const WIN_TITLE: &str = "polynomial-renderer";

fn main() {
	let mut win = RenderWindow::new(WIN_SIZE, WIN_TITLE,
		Default::default(), &Default::default());
    
    win.set_vertical_sync_enabled(true);
    
    let mut factor = 0.5;
	
    'game: loop {
		let mut vtx_arr = VertexArray::new(PrimitiveType::Points, 16);
		
		let pos = ((200., 200.), (400., 400.));
		vtx_arr.append(&vtx(pos.0));
		vtx_arr.append(&vtx(pos.1));
		
		let point1 = interpolate(factor, pos.0, pos.1);
		vtx_arr.append(&point1);
		
		let pos = ((200., 200.), (100., 300.));
		vtx_arr.append(&vtx(pos.0));
		vtx_arr.append(&vtx(pos.1));
		
		let point2 = interpolate(1.0 - factor, pos.0, pos.1);
		vtx_arr.append(&point2);
		
		let final_point = interpolate(1.0 - factor, point1.position, point2.position);
		vtx_arr.append(&final_point);
		
		win.clear(&Color::BLACK);
		win.draw(&vtx_arr);
		win.display();
		
		while let Some(ev) = win.poll_event() {
			match ev {
				Event::KeyPressed { code: Key::Escape, .. }
						=> break 'game,
				Event::KeyPressed { code, .. } => {
					match code {
						Key::Up => factor += 0.025,
						Key::Down => factor -= 0.025,
						_ => {},
					}
					
					factor = factor.max(0.0).min(1.0);
					println!("{}", factor);
				},
				Event::Closed => break 'game,
				_ => {},
			}
		}
	}
}

fn vtx<V: Into<Vector2f>>(coords: V) -> Vertex {
	Vertex {
		position: coords.into(),
		..Default::default()
	}
}

fn interpolate<V: Into<Vector2f>>(factor: f32, a: V, b: V) -> Vertex {
	let (a, b) = (a.into(), b.into());
	
	let factor_a = 1.0 - factor;
	let factor_b = factor;
	
	vtx(a * factor_a + b * factor_b)
}
