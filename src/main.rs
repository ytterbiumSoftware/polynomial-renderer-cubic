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
const STEPS: u32 = 100;

fn main() {
    let settings = ContextSettings {
        antialiasing_level: 8,
        ..Default::default()
    };
    
	let mut win = RenderWindow::new(WIN_SIZE, WIN_TITLE,
		Default::default(), &settings);
	
	win.set_vertical_sync_enabled(true);
	
	let mut curve = render_curve(&[
		(0.,   400.),
		(200., 200.),
		(600., 600.),
		(800., 400.),
	]);
    
    curve.push(Vector2f::new(0.,   1200.));
    curve.push(Vector2f::new(800., 1200.));
	
    let shape = create_shape(&curve);
    
	'game: loop {
		win.clear(&Color::BLACK);
        win.draw_convex_shape(&shape, Default::default());
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

fn render_curve<V>(points: &[V; 4]) -> Vec<Vector2f>
    where V: Into<Vector2f> + Copy {
    
	let mut curve = Vec::new();
    
	for f in 0..(STEPS + 1) {
		let factor = f as f32 / STEPS as f32;
		
		let point1a = interpolate(factor, points[1], points[0]);
		let point2a = interpolate(factor, points[2], points[1]);
        let point3a = interpolate(factor, points[3], points[2]);
        
        let point1b = interpolate(factor, point2a, point1a);
        let point2b = interpolate(factor, point3a, point2a);
		
		let curve_point = interpolate(factor, point2b, point1b);
		curve.push(curve_point);
	}
	
	/*for i in points {
		vtx_arr.append(&vtx_color(*i, POINT_COLOR))
	}*/
    
    //let c = vtx_arr.vertex_count() - 1;
    //vtx_arr.resize(c);
    
    //for i in vtx_arr.vertices() {
    //    println!("{:?}", *i);
    //}
	
	curve
}

fn interpolate<V: Into<Vector2f>>(factor: f32, a: V, b: V) -> Vector2f {
	let (a, b) = (a.into(), b.into());
	
	let factor_a = 1.0 - factor;
	let factor_b = factor;
	
	a * factor_a + b * factor_b
}

fn create_shape(vertices: &[Vector2f]) -> ConvexShape {
    let mut s = ConvexShape::new(vertices.len() as u32);
    s.set_outline_color(&PATH_COLOR);
    
    for (i, val) in vertices.iter().enumerate() {
        s.set_point(i as u32, *val)
    }
    
    s
}
