/* Define a trait called Drawable with a method draw. Create a struct called
Shape that stores the name of a shape and implement the Drawable trait for
it. */
trait Drawable {
    fn draw(&self);
}

struct Shape {
    name: String,
}

// Implement the Drawable trait for the Shape struct
impl Drawable for Shape {
    fn draw(&self) {
        println!("Drawing {} shape.", self.name);
    }
}

fn main() {
    let circle = Shape {
        name: String::from("Circle"),
    };

    let rectangle = Shape {
        name: String::from("Rectangle"),
    };

    circle.draw();
    rectangle.draw();
}
