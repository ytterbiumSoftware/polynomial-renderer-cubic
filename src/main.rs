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
	
	let vertex_array = render_curve(&[
		Vector2f::new(200., 200.),
		Vector2f::new(100., 300.),
		Vector2f::new(400., 400.),
	]);
	
	'game: loop {
		win.clear(&Color::BLACK);
		win.draw_vertex_array(&vertex_array, Default::default());
		win.display();
		
		while let Some(ev) = win.poll_event() {
			match ev {
				Event::KeyPressed { code: Key::Escape, .. }
					=> break 'game,
				Event::Closed => break 'game,
				_ => {},
			}
		}
	}
}

fn render_curve(points: &[Vector2f; 3]) -> VertexArray {
	let mut vtx_arr = VertexArray::new(PrimitiveType::Points, 1024);
	
	for f in 0..101 {
		let factor = f as f32 / 100.;
		
		println!("{:?}", factor);
		
		let point1 = interpolate_magenta(factor, points[0], points[1]);
		vtx_arr.append(&point1);
		
		let point2 = interpolate_magenta(factor, points[2], points[0]);
		vtx_arr.append(&point2);
		
		let curve_point = interpolate_cyan(1.0 - factor,
			point1.position, point2.position);
		vtx_arr.append(&curve_point);
	}
	
	for i in points {
		vtx_arr.append(&vtx(*i))
	}
	
	vtx_arr
}

fn interpolate<V: Into<Vector2f>>(factor: f32, a: V, b: V) -> Vertex {
	let (a, b) = (a.into(), b.into());
	
	let factor_a = 1.0 - factor;
	let factor_b = factor;
	
	vtx(a * factor_a + b * factor_b)
}

fn interpolate_cyan<V: Into<Vector2f>>(factor: f32, a: V, b: V) -> Vertex {
	let (a, b) = (a.into(), b.into());
	
	let factor_a = 1.0 - factor;
	let factor_b = factor;
	
	vtx_cyan(a * factor_a + b * factor_b)
}

fn interpolate_magenta<V: Into<Vector2f>>(factor: f32, a: V, b: V) -> Vertex {
	let (a, b) = (a.into(), b.into());
	
	let factor_a = 1.0 - factor;
	let factor_b = factor;
	
	vtx_magenta(a * factor_a + b * factor_b)
}

fn vtx<V: Into<Vector2f>>(coords: V) -> Vertex {
	Vertex {
		position: coords.into(),
		..Default::default()
	}
}

fn vtx_cyan<V: Into<Vector2f>>(coords: V) -> Vertex {
	Vertex {
		position: coords.into(),
		color: Color::CYAN,
		..Default::default()
	}
}

fn vtx_magenta<V: Into<Vector2f>>(coords: V) -> Vertex {
	Vertex {
		position: coords.into(),
		color: Color::MAGENTA,
		..Default::default()
	}
}
