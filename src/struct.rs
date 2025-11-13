fn main() {
    struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    }

    let mut user1 = User {
        username: String::from("user1"),
        email: String::from("test@test.com"),
        sign_in_count: 1,
        active: true,
    };  

    println!("Username: {}", user1.username);
    user1.username = String::from("new_user1");
    println!("Username: {}", user1.username);
    
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn square(size: u32) -> Self{
            Rectangle {
                width: size,
                height: size,
            }
        } 
        
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is: {}", rect1.area());
}