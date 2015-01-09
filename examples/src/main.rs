extern crate paint;
use paint::{ Layer, Canvas, Projection, Color, Size };

struct BigWhiteCircle;

impl Layer for BigWhiteCircle{
	fn draw(&self, projection :Projection)->Color{
		use std::cmp::{ max, min };
		let origin_x = std::u32::MAX / 2;
		let offset_x = max(projection.x, origin_x) - min(projection.x, origin_x);
		let origin_y = std::u32::MAX / 2;
		let offset_y = max(projection.y, origin_y) - min(projection.y, origin_y);
		let offset_x = offset_x as f64;
		let offset_y = offset_y as f64;
		let dis = std::num::Float::sqrt(offset_x * offset_x + offset_y * offset_y);
		if dis < (std::u32::MAX / 2) as f64{
			Color::rgb(255, 255, 255)
		}else{
			Color::transparent()
		}
	}
}

struct Position(f32, f32, f32);

struct Light{
	origin :Position,
}

struct Ball{
	origin :Position,
	radius :f32
}

struct Scene{
	light :Light,
	ball :Ball,
	eye :Position
}

impl Scene{
	fn new()->Scene{
		let ball = Ball{ origin: Position(3f32, 30f32, 3f32), radius: 3f32 };
		let light = Light{ origin: Position(0f32, 0f32, 0f32) };
		let eye = Position(3f32, 0f32, 3f32);
		Scene{ ball: ball, light: light, eye: eye }
	}
}

struct TwoLayer<U :Layer, V :Layer>{
	a :U, b :V
}

impl<U :Layer, V :Layer> Layer for TwoLayer<U, V>{
	fn draw(&self, projection :Projection)->Color{
		projection.proxy_split(|p|self.a.draw(p), |p|self.b.draw(p))
	}
}

fn main(){
	let args = std::os::args();
	if args.len() == 2{
		match std::path::Path::new_opt(&*args[1]){
			Some(ref path) =>{
				match std::io::fs::File::create(path){
					Ok(ref mut file) =>{
						let canvas = Canvas::new(Color::rgb(0, 200, 0), Size::new(260, 520));
						let layer = TwoLayer{ a: BigWhiteCircle, b: BigWhiteCircle };
						canvas.render(layer, file);
					},
					Err(x)=>println!("{}", x)
				}
			},
			None =>println!("failed to open path")
		}
	}else{
		println!("Usage: {} file_path \t write a ppm file", &*args[0])
	}
}