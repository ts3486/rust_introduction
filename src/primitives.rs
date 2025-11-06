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
}
