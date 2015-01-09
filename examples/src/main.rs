extern crate getopts;
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

fn main(){
	let args = std::os::args();
	if args.len() == 2{
		match std::path::Path::new_opt(&*args[1]){
			Some(ref path) =>{
				match std::io::fs::File::create(path){
					Ok(ref mut file) =>{
						let canvas = Canvas::new(Color::rgb(0, 200, 0), Size::new(480, 360));
						canvas.render(BigWhiteCircle, file);
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