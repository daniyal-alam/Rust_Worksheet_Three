/* Design a geometry library in Rust. Create traits like Area and Perimeter
that have methods for calculating area and perimeter for various geometric
shapes (e.g., Circle, Rectangle). Implement these traits for the respective
struct types.
 */
trait Area {
    fn area(&self) -> f64;
}

trait Perimeter {
    fn perimeter(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Perimeter for Circle {
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Perimeter for Rectangle {
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };

    println!("Circle Area: {:.2}", circle.area());
    println!("Circle Perimeter: {:.2}", circle.perimeter());

    println!("Rectangle Area: {:.2}", rectangle.area());
    println!("Rectangle Perimeter: {:.2}", rectangle.perimeter());
}
