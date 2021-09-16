#[derive(Debug)]
struct ShapeInfo{
	width : u32,
	heigth : u32,
}

impl ShapeInfo{

	fn crate_square(size : u32) -> ShapeInfo{
		ShapeInfo{
			width : size,
			heigth :size,
		}
	}

	fn area(&self) -> u32{
		self.width * self.heigth
	}

	fn scale(&mut self , value : u32) {
		self.heigth *= value;
		self.width *= value;
	}
}

pub fn structs(){
	let mut square = ShapeInfo::crate_square(32);

	println!("The area for shape {:#?} is {}", square, square.area());

	square.scale(2);

	println!("The area for shape {:#?} is {}", square, square.area());
}