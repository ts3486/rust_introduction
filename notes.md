<!-- hello world -->

<!-- fn main() {
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
} -->



<!-- primitives -->
<!-- 
fn main() {
    println!("Hello, world!");

     // Rust types

    // BOOLEAN TYPE
    let _boolean: bool = true;

    // SCALAR TYPES
    let _integer: i8 = -100;
    let _unsigned_integer: u8 = 200;
    let _integer_16: i16 = -20000;
    let _unsigned_integer_16: u16 = 40000;
    let _integer_32: i32 = -100000;
    let _unsigned_integer_32: u32 = 3000000000;
    let _integer_64: i64 = -5000000000;
    let _unsigned_integer_64: u64 = 10000000000;
    let _isize_integer: isize = -123456;
    let _usize_integer: usize = 123456;


    let _float_32: f32 = 3.14;
    let _float_64: f64 = 2.718281828459045;

    let x: usize = 42; // i32 by default
    let y: f32 = 3.14; // f64 by default

    // let z = x / y; // This line would cause a compile-time error due to type mismatch
    let z = x as f32 / y; // Type casting
    println!("The value of z is: {}", z);

    // CHARACTER TYPE
    let _character: char = 'R';

    // COMPOUND TYPES
    // TUPLE TYPE
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tuple;
    println!("The value of b in the tuple is: {}", b);


    // ARRAY TYPE
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let first_element = array[0];
    println!("The first element of the array is: {}", first_element);
    // if you try to access an index out of bounds, it will cause a compile-time error (e.g panic at runtime)
} -->

<!-- functions -->

<!-- fn main() {
    println!("Hello, world!");

    // FUNCTION DEFINITIONS AND CALLS
    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }

    greet("Alice");

    let y = {
        let x = 5;
        x + 10 // The last expression is returned when written without a semicolon
    };

    println!("The value of y is: {}", y);

    // IF EXPRESSIONS
    let number = 7;
    
    // - No need for parentheses around the condition. 
    // - The value must be a boolean.
    // - The type of the if and return value must be the same.
    if number < 5 {
        println!("The number is less than 5");
    } else if number == 5 {
        println!("The number is equal to 5");
    } else {
        println!("The number is greater than 5");
    }

    // LOOPING WITH LOOP, WHILE, AND FOR
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break;
        }
        println!("Count in loop: {}", count);
    }   
    let mut n = 1;
    while n < 4 {
        println!("n in while: {}", n);
        n += 1;
    }

    let array = [10, 20, 30, 40, 50];
    for element in array.iter() {
        println!("Element in for: {}", element);
    }
    for element in (1..4) {
        println!("Element in for for: {}", element);
    }
}
 -->

 <!-- ownership -->

<!-- fn main() {
    println!("Hello, world!");

    // Stack
    // memory allocated near the variables and is faster but less flexible.

    // Heap
    // memory is allocated from the OS and is more flexible but slower.

    // OWNERSHIP RULES
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
}
 -->