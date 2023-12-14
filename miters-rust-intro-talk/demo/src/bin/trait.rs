#![allow(dead_code)]

use std::f64::consts;

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        consts::PI * self.radius * self.radius
    }
}

//Trait bound
fn print_shape<T>(shape: T)
where
    T: Shape,
{
    println!("{}", shape.area())
}

struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let square = Square { side: 5.0 };

    println!("Area of circle: {}", circle.area());
    println!("Area of square: {}", square.area());
}
