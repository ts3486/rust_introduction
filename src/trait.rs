// TRAITS: A trait defines shared behavior that multiple types can implement
// Think of it like an interface or contract that says "if you implement this trait, you must have these methods"

// 1. DEFINING A TRAIT
trait Drawable {
    fn draw(&self);
    fn describe(&self) -> String;
}

// 2. IMPLEMENTING A TRAIT FOR A STRUCT
struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle");
    }

    fn describe(&self) -> String {
        format!("Circle with radius: {}", self.radius)
    }
}

// 3. IMPLEMENTING THE SAME TRAIT FOR ANOTHER STRUCT
struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle");
    }

    fn describe(&self) -> String {
        format!("Rectangle: {} x {}", self.width, self.height)
    }
}

// 4. USING TRAIT OBJECTS - allows different types to be treated the same way
fn display_shape(shape: &dyn Drawable) {
    shape.draw();
    println!("{}", shape.describe());
}

// 5. USING GENERICS WITH TRAIT BOUNDS
fn print_drawable<T: Drawable>(item: &T) {
    item.draw();
    println!("{}", item.describe());
}

// 6. DEFAULT IMPLEMENTATIONS IN TRAITS
trait Animal {
    fn name(&self) -> &str;

    // This method has a default implementation
    fn speak(&self) {
        println!("{} makes a sound", self.name());
    }
}

// 7. MULTIPLE TRAIT BOUNDS WITH THE + OPERATOR
// The + syntax means "must implement BOTH traits"
use std::fmt::Display;

trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.content)
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.headline)
    }
}

// Function that requires BOTH Summary AND Display traits
// The + means the type must implement both traits
fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item); // Uses Display
    println!("Summary: {}", item.summarize()); // Uses Summary
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }

    // Can override the default
    fn speak(&self) {
        println!("{} barks!", self.name());
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }
    // Uses the default speak() implementation
}

fn main() {
    println!("=== TRAIT EXAMPLE ===\n");

    // Creating instances
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 10.0,
        height: 20.0,
    };

    // Using trait objects (same interface, different types)
    println!("--- Using trait objects ---");
    display_shape(&circle);
    println!();
    display_shape(&rectangle);
    println!();

    // Using generics with trait bounds
    println!("--- Using generics with trait bounds ---");
    print_drawable(&circle);
    println!();
    print_drawable(&rectangle);
    println!();

    // Using traits with default implementations
    println!("--- Using traits with default implementations ---");
    let dog = Dog {
        name: String::from("Buddy"),
    };
    let cat = Cat {
        name: String::from("Whiskers"),
    };

    dog.speak(); // Custom implementation
    cat.speak(); // Uses default implementation
    println!();

    // Using multiple trait bounds with the + operator
    println!("--- Using multiple trait bounds (+ operator) ---");
    let article = NewsArticle {
        headline: String::from("Rust is Awesome!"),
        content: String::from("Learn Rust with traits and generics."),
    };
    notify(&article);
}