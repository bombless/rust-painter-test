use std::io::Writer;

#[derive(Copy)]
pub struct Size{
	width :u32,
	height :u32
}

impl Size{
	pub fn new(w :u32, h :u32)->Size{
		Size{ width: w, height: h }
	}
}

#[derive(Copy)]
pub struct Position{
	x :f32,
	y :f32
}

impl Position{
	pub fn new(x :u32, y:u32)->Position{
		Position{ x: x as f32, y: y as f32 }
	}
	
	pub fn distance(&self, point :Position)->f32{
		use std::num::Float;
		let square = |&: x :f32| x * x;
		let offset_x = self.x - point.x;
		let offset_y = self.y - point.y;
		Float::sqrt(square(offset_x) + square(offset_y))
	}
}

#[derive(PartialEq, Copy)]
pub struct Color(f32, f32, f32, f32);

impl Color{
	fn f32_to_u8(v :f32)->u8{
		use std::num::ToPrimitive;
		match ToPrimitive::to_u8(&(v * 256f32)){
			Some(x) =>x,
			None =>if v > 0.5f32{ 255 }else{ 0 }
		}		
	}
	
	pub fn r(&self)->u8{ Color::f32_to_u8(self.0) }
	
	pub fn g(&self)->u8{ Color::f32_to_u8(self.1) }
	
	pub fn b(&self)->u8{ Color::f32_to_u8(self.2) }
	
	pub fn rgb(r :u8, g :u8, b :u8)->Color{
		Color::rgba(r, g, b, 255)
	}
	
	pub fn rgba(r :u8, g :u8, b :u8, a :u8)->Color{
		Color(r as f32 / 256f32, g as f32 / 256f32, b as f32 / 256f32, a as f32 / 256f32)
	}
	
	pub fn transparent()->Color{
		Color(1f32, 1f32, 1f32, 0f32)
	}
	
	pub fn over(&self, dst :Color)->Color{
		let &Color(lhs_r, lhs_g, lhs_b, lhs_a) = self;
		let Color(rhs_r, rhs_g, rhs_b, rhs_a) = dst;
		let r = lhs_r * lhs_a + rhs_r * rhs_a * (1f32 - lhs_a);
		let g = lhs_g * lhs_a + rhs_g * rhs_a * (1f32 - lhs_a);
		let b = lhs_b * lhs_a + rhs_b * rhs_a * (1f32 - lhs_a);
		let a = if lhs_a > rhs_a { lhs_a }else { rhs_a };
		Color(r, g, b, a)
	}
}

#[derive(Copy)]
pub struct Projection{
	pub x :u32,
	pub y :u32
}

impl Projection{
	fn f64_to_u32(v :f64)->u32{
		use std::u32;
		use std::f64;
		use std::num::ToPrimitive;
		match ToPrimitive::to_u32(&v){
			Some(v) =>v,
			None =>if v > f64::MAX_VALUE / 2f64{
				u32::MAX
			}else{
				0
			}
		}
	}
	
	pub fn proxy_split<F1 :Fn(Projection)->Color, F2 :Fn(Projection)->Color>(&self, f1 :F1, f2 :F2)->Color{
		use std::u32;
		use std::num::ToPrimitive;
		let mid_value = (u32::MAX / 2) as f64;
		let (value, latter) = if self.y > u32::MAX / 2{
			(2f64 * (self.y as f64 - mid_value), true)
		}else{
			(2f64 * self.y as f64, false)
		};
		let p = match ToPrimitive::to_u32(&value){
			Some(y) =>Projection{ x: self.x, y: y },
			None =>Projection{ x: self.x, y: if value > mid_value{
					u32::MAX
				}else{
					0
				}
			}
		};
		if latter{ f2(p) }else{ f1(p) }
	}
	
	pub fn new(canvas_size :&Size, position :Position)->Projection{
		use std::u32;
		let relative_x = position.x as f64 / canvas_size.width as f64;
		let x = Projection::f64_to_u32(relative_x * u32::MAX as f64);
		let relative_y = position.y as f64 / canvas_size.height as f64;
		let y = Projection::f64_to_u32(relative_y * u32::MAX as f64);
		Projection{ x: x, y: y }
	}
}

pub trait Layer{
	fn draw(&self, Projection)->Color;
}

#[derive(Copy)]
pub struct Canvas{
	background :Color,
	size :Size
}

impl Canvas{
	pub fn new(background :Color, s :Size)->Canvas{
		Canvas{ size: s, background: background }
	}
	
	pub fn render<T :Layer>(&self, layer :T, writer :&mut Writer){
		let mut canvas = Vec::new();
		for _ in 0 .. self.size.width * self.size.height{
			canvas.push(self.background)
		}
		for x in 0 .. self.size.width{
			for y in 0 .. self.size.height{
				let index = x as usize + self.size.width as usize * y as usize;
				let projection = Projection::new(&self.size, Position::new(x, y));
				canvas[index] = layer.draw(projection).over(canvas[index])
			}
		}
		writer.write_line("P3").unwrap();
		writer.write_line(&*format!("{} {}", self.size.width, self.size.height)).unwrap();
		writer.write_line("255").unwrap();
		for i in canvas.iter(){
			writer.write_line(&*format!("{} {} {}", i.r(), i.g(), i.b())).unwrap();
		}
	}
}