extern crate sfml;

use sfml::graphics::*;
use sfml::system::Vector2f;
use sfml::window::*;

const WIN_SIZE: (u32, u32) = (800, 600);
const WIN_TITLE: &str = "polynomial-renderer";
//const POINT_COLOR: Color = Color::CYAN;
//const LEG1_COLOR: Color = Color::MAGENTA;
//const LEG2_COLOR: Color = Color::YELLOW;
const PATH_COLOR: Color = Color::WHITE;

fn main() {
	let mut win = RenderWindow::new(WIN_SIZE, WIN_TITLE,
		Default::default(), &Default::default());
	
	win.set_vertical_sync_enabled(true);
	
	let vertex_array = render_curve(&[
		Vector2f::new(100., 100.),
		Vector2f::new(700., 200.),
		Vector2f::new(150., 350.),
		Vector2f::new(550., 500.),
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

fn render_curve(points: &[Vector2f; 4]) -> VertexArray {
	let mut vtx_arr = VertexArray::new(PrimitiveType::LineStrip, 0);
	
	for f in 0..101 {
		let factor = f as f32 / 100.;
		
		println!("{:?}", factor);
		
		let point1a = interpolate(factor, points[1], points[0]);
        //vtx_arr.append(&vtx_color(point1a, LEG1_COLOR));
		
		let point2a = interpolate(factor, points[2], points[1]);
        //vtx_arr.append(&vtx_color(point2a, LEG1_COLOR));
        
        let point3a = interpolate(factor, points[3], points[2]);
        //vtx_arr.append(&vtx_color(point3a, LEG1_COLOR));
        
        let point1b = interpolate(factor, point2a, point1a);
        //vtx_arr.append(&vtx_color(point1b, LEG2_COLOR));
        
        let point2b = interpolate(factor, point3a, point2a);
        //vtx_arr.append(&vtx_color(point2b, LEG2_COLOR));
		
		let curve_point = interpolate(factor, point2b, point1b);
		vtx_arr.append(&vtx(curve_point));
	}
	
	/*for i in points {
		vtx_arr.append(&vtx_color(*i, POINT_COLOR))
	}*/
    
    //let c = vtx_arr.vertex_count() - 1;
    //vtx_arr.resize(c);
    
    //for i in vtx_arr.vertices() {
    //    println!("{:?}", *i);
    //}
	
	vtx_arr
}

fn interpolate<V: Into<Vector2f>>(factor: f32, a: V, b: V) -> Vector2f {
	let (a, b) = (a.into(), b.into());
	
	let factor_a = 1.0 - factor;
	let factor_b = factor;
	
	a * factor_a + b * factor_b
}

fn vtx<V: Into<Vector2f>>(coords: V) -> Vertex {
	vtx_color(coords, PATH_COLOR)
}

fn vtx_color<V: Into<Vector2f>>(coords: V, color: Color) -> Vertex {
    Vertex {
        position: coords.into(),
        color,
        .. Default::default()
    }
}
