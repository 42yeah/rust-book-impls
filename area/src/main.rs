use std::fmt::Display;

struct Rect {
    width: i32,
    height: i32
}

impl Rect {
    pub fn new(width: i32, height: i32) -> Rect {
        Rect {
            width, height
        }
    }

    fn area(&self) -> i32 {
        self.width * self.height
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rect with width: {} and height: {}", self.width, self.height)
    }
}

fn main() {
    println!("{}", Rect::new(10, 10));

    let square = Rect::new(20, 20);
    println!("Area of {} is: {}", square, square.area());
}
