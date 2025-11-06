fn main() {
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
