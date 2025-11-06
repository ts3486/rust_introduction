fn main() {
    println!("Hello, world!");

    // By default variables are immutable
    let x: i32 = 5;
    println!("The value of x is: {}", x);
    // x = 6; // This line would cause a compile-time error because x is immutable

    // Mutable variables
    let mut y: i32 = 10;
    println!("The value of y is: {}", y);

    y = 15; 
    println!("The value of y is: {}", y);

    // Constants HAVE to have a value during compile time
    const CONSTANT: usize = 100;
    println!("The value of CONSTANT is: {}", CONSTANT);


    let z: i32 = 20;
    println!("The value of z is: {}", z);
    let z: i32 = 25; // Shadowing: creating a new variable with the same name
    println!("The value of z after shadowing is: {}", z);

    // scope
    {
        let a: i32 = 30;
        let z: i32 = 35; // Shadowing in inner scope
        println!("The value of a in the inner scope is: {}", a);
        println!("The value of z in the inner scope is: {}", z);
    }

    println!("The value of z in the outer scope is: {}", z);

    let string: &str = "Hello, Rust!";
    println!("The value of string is: {}", string);
    println!("The length of string is: {}", string.len());
}
