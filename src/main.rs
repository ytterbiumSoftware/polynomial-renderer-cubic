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
const STEPS_QUADRATIC: u32 = 50;
const STEPS_CUBIC: u32 = 100;
const TEXTURE_PATH: &str = "ground.png";

fn main() {
    let settings = ContextSettings {
        antialiasing_level: 8,
        ..Default::default()
    };
    
	let mut win = RenderWindow::new(WIN_SIZE, WIN_TITLE,
		Default::default(), &settings);
	
	win.set_vertical_sync_enabled(true);
    
    let tex = Texture::from_file(TEXTURE_PATH).unwrap();
	
    //let mut curve = render_line(&[(0., 500.), (800., 300.)]);
    
    /*
    let mut curve = render_curve_quadratic(&[
        (0.,   400.),
        (100., 300.),
        (800., 400.),
    ]);
    */
    
	let mut curve = render_curve_cubic(&[
		(0.,   400.),
		(200., 500.),
		(600., 300.),
		(800., 400.),
	]);
    
    curve.push(Vector2f::new(800., 1200.));
    curve.push(Vector2f::new(0.,   1200.));
    
    let shape = create_shape(&curve, &tex);
    
	'game: loop {
		win.clear(&Color::BLACK);
        win.draw_convex_shape(&shape, Default::default());
        //win.draw_vertex_array(&vtx_arr, Default::default());
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

fn render_line<V>(points: &[V; 2]) -> Vec<Vector2f>
    where V: Into<Vector2f> + Copy {
    
    vec![points[0].into(), points[1].into()]
}

fn render_curve_quadratic<V>(points: &[V; 3]) -> Vec<Vector2f>
    where V: Into<Vector2f> + Copy {
    
    let mut curve = Vec::new();
    
    for f in 0..(STEPS_QUADRATIC + 1) {
        let factor = f as f32 / STEPS_QUADRATIC as f32;
        
        let point1 = interpolate(factor, points[1], points[2]);
        let point2 = interpolate(factor, points[0], points[1]);
        
        let curve_point = interpolate(factor, point2, point1);
        curve.push(curve_point);
    }
    
    curve
}

fn render_curve_cubic<V>(points: &[V; 4]) -> Vec<Vector2f>
    where V: Into<Vector2f> + Copy {
    
	let mut curve = Vec::new();
    
	for f in 0..(STEPS_CUBIC + 1) {
		let factor = f as f32 / STEPS_CUBIC as f32;
		
		let point1a = interpolate(factor, points[2], points[3]);
		let point2a = interpolate(factor, points[1], points[2]);
        let point3a = interpolate(factor, points[0], points[1]);
        
        let point1b = interpolate(factor, point2a, point1a);
        let point2b = interpolate(factor, point3a, point2a);
		
		let curve_point = interpolate(factor, point2b, point1b);
		curve.push(curve_point);
	}
	
	curve
}

fn interpolate<V: Into<Vector2f>>(factor: f32, a: V, b: V) -> Vector2f {
	let (a, b) = (a.into(), b.into());
	
	let factor_a = 1.0 - factor;
	let factor_b = factor;
	
	a * factor_a + b * factor_b
}

fn create_shape<'a>(vertices: &[Vector2f], texture: &'a TextureRef) -> ConvexShape<'a> {
    let mut s = ConvexShape::with_texture(vertices.len() as u32, texture);
    s.set_outline_color(&PATH_COLOR);
    
    for (i, val) in vertices.iter().enumerate() {
        s.set_point(i as u32, *val)
    }
    
    s
}
