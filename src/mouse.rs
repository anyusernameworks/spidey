pub struct Mouse {
	left: bool,
	x: f64,
	y: f64,
	dx: f64,
	dy: f64,
}
impl Mouse {
	pub fn new() -> Mouse {
		Mouse {
			left: false,
			x: 0.0,
			y: 0.0,
			dx: 0.0,
			dy: 0.0,
		}
	}
	pub fn set_left(&mut self, a: bool) {
		self.left = a;
	}
	pub fn left(&self) -> bool {
		self.left
	}
	pub fn set_pos(&mut self, a: (f64, f64)) {
		self.x = a.0;
		self.y = a.1;
	}
	pub fn pos(&self) -> (f64, f64) {
		(self.x,self.y)
	}
	pub fn set_d_pos(&mut self, a: (f64, f64)) {
		self.dx = a.0;
		self.dy = a.1;
	}
	pub fn d_pos(&self) -> (f64, f64) {
		(self.dx,self.dy)
	}
	pub fn reset(&mut self) {
		self.dx = 0.0;
		self.dy = 0.0;
	}
}