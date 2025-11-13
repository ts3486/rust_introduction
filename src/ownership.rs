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

    { // Start of a new scope
        let s: &str = "hello"; // s is valid from this point forward
        println!("The value of s is: {}", s); // s is valid until the end of this scope
    } // End of the scope, s is no longer valid

    // println!("The value of s is: {}", s); // This line would cause a compile-time error

    // String TYPE AND OWNERSHIP

    // String is a heap-allocated, and can be modified unlike &str.
    {
        let s: String = String::from("hello"); // s is valid from this point forward
        println!("The value of s is: {}", s); // s is valid until the end of this scope
    } // End of the scope, the memory of s is returned to the OS.

    // MOVING OWNERSHIP
    {
        let s1: String = String::from("hello");
        let s2: String = s1; // s1's value is moved to s2, s1 is no longer valid

        // println!("The value of s1 is: {}", s1); // This line would cause a compile-time error
        println!("The value of s2 is: {}", s2);
        // println!("The value of s1 is: {}", s1); // This line would cause a compile-time error
    }

    // IMPORTANT: stack allocated data (like &str) is stored directly in the variable, and does not move ownsership.

    // CLONING DATA
    // If you want to keep using the original variable after moving the ownership, you can use the clone method to create a deep copy of the data.
    {
        let s1: String = String::from("hello");
        let s2: String = s1.clone(); // s1 is cloned to s2, both are valid

        println!("The value of s1 is: {}", s1);
        println!("The value of s2 is: {}", s2);
    }

    // OWNERSHIP AND FUNCTIONS
    {
        fn takes_ownership(s: String) {
            println!("The value of s is: {}", s);
        } // s goes out of scope and its memory is returned to the OS

        let s: String = String::from("hello");
        takes_ownership(s); // s's ownership is moved to the function

        // println!("The value of s is: {}", s); // This line would cause a compile-time error
    }

    {
        fn makes_copy(x: i32) {
            println!("The value of x is: {}", x);
        } // x goes out of scope, but nothing special happens

        let x: i32 = 5;
        makes_copy(x); // x is copied to the function

        println!("The value of x is: {}", x); // x is still valid
    }

    // RETURNING OWNERSHIP
    {
        fn gives_ownership() -> String {
            let s: String = String::from("hello");
            s // s is returned and its ownership is moved to the caller
        }

        let s1: String = gives_ownership(); // s1 now owns the value returned by gives_ownership

        println!("The value of s1 is: {}", s1);
    }

    {
        fn takes_and_gives_back(s: String) -> String {
            s // s is returned and its ownership is moved back to the caller
        }

        let s2: String = String::from("hello");
        let s3: String = takes_and_gives_back(s2); // s2's ownership is moved to the function and then back to s3

        // println!("The value of s2 is: {}", s2); // This line would cause a compile-time error
        println!("The value of s3 is: {}", s3);
    }

    // BORROWING WITH REFERENCES
    {
        fn calculate_length(s: &String) -> usize {
            s.len() // we can use s but we do not own it
        } // s goes out of scope, but nothing special happens

        let s1: String = String::from("hello");
        let len: usize = calculate_length(&s1); // we pass a reference to s1

        println!("The length of '{}' is {}.", s1, len); // s1 is still valid
    }

    
}
