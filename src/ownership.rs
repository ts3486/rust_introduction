fn main() {
    println!("Hello, world!");

    // Stack
    // memory allocated near the variables and is faster but less flexible.

    // Heap
    // memory is allocated from the OS and is more flexible but slower.

    // OWNERSHIP RULES
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
}
