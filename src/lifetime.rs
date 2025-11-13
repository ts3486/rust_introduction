// LIFETIMES: short, focused examples and explanations
// This file demonstrates when lifetime annotations are required,
// how to write a function that returns a reference, structs with
// lifetime parameters, 'static lifetime and elision rules.

// Example 1: function returning the longest of two string slices
// This requires lifetimes because the returned reference must be
// guaranteed to be valid as long as the inputs.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Example 2: lifetime annotations on structs
// A struct that holds a reference needs a lifetime parameter so
// the compiler knows how long the reference inside the struct is valid.

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // This method takes a string slice `announcement` and returns
    // a reference to the `part` field. The output lifetime is the
    // same as the lifetime of `self` (elision rules help here).
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Announcement: {}", announcement);
        self.part
    }
}

// Example 3: 'static lifetime
// String literals have a 'static lifetime - they live for the entire program run.

fn static_example() {
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);
}

// Example 4: lifetime elision rules
// In many cases you don't need to write lifetime annotations because
// the compiler applies elision rules. For example:

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// Example 5: mixing owned and borrowed values
fn demo_mixed() {
    let string1 = String::from("abcd");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest is: {}", result);
    } // <- string2 goes out of scope here
      // result would not be usable here if it referred to string2
}

// Example 5b: WHY THIS WOULD CAUSE A COMPILE ERROR
// This function demonstrates a lifetime violation. The compiler will REJECT this code.
// 
// The problem:
// 1. longest<'a>(...) -> &'a str requires both input lifetimes to be 'a
// 2. string1 lives for the entire function (outer scope)
// 3. string2 lives only inside the inner block { }
// 4. The compiler tries to find a lifetime 'a that works for BOTH:
//    - 'a must be at least as long as string1's lifetime (outer scope)
//    - 'a must be at least as long as string2's lifetime (inner scope only)
// 5. But 'a can't satisfy both! string2 dies when the block ends, so 'a can't extend into the outer scope
// 6. result tries to hold a reference with lifetime 'a, but 'a doesn't exist
//
// COMPILE ERROR: the lifetime of the returned reference cannot be determined
// because string2 (which might be returned) is only valid inside the block,
// but result outlives the block.
//
// WHAT THE COMPILER SAYS (approximately):
// "cannot return reference to local variable `string2`"
// "string2 is borrowed, and the borrow does not live long enough"
//
fn demo_mixed_two() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        // This line violates the lifetime contract:
        result = longest(string1.as_str(), string2.as_str());
        // longest() tries to return a &'a str where 'a covers both inputs,
        // but string2 only lives inside this block!
        println!("The longest is: {}", result);
    } // <- string2 is dropped here
      // result now holds a dangling reference if it pointed to string2
      // This is memory-unsafe, so Rust prevents it at compile time
    
    // If we tried to use result here, it would definitely be invalid:
    // println!("{}", result);  // <- COMPILER ERROR: use of partially moved value
}


fn main() {
    println!("=== LIFETIMES DEMO ===\n");

    // longest
    let a = String::from("long string is long");
    let b = "xyz";
    let result = longest(a.as_str(), b);
    println!("longest: {}\n", result);

    // ImportantExcerpt
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("couldn't find a '.'");
    let excerpt = ImportantExcerpt { part: first_sentence };
    println!("excerpt part: {}", excerpt.part);
    println!("excerpt level: {}", excerpt.level());
    println!("announce_and_return_part: {}\n", excerpt.announce_and_return_part("note:"));

    // 'static
    static_example();
    println!();

    // elision example
    let s = String::from("hello world");
    let fw = first_word(&s);
    println!("first_word: {}\n", fw);

    // mixing owned/borrowed
    demo_mixed();

    println!("Demo complete.");
}