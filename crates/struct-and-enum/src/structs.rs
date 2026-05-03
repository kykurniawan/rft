#[cfg(test)]
mod tests {
    #[test]
    fn define_struct() {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 0 };
        println!("Point: ({}, {})", p.x, p.y);

        struct User {
            name: String,
            email: String,
            is_active: bool,
        }

        let mut user = User {
            name: String::from("John"),
            email: String::from("john@example.com"),
            is_active: true,
        };

        println!("User Name: {}", user.name);
        println!("User Email: {}", user.email);
        println!("User Is Active: {}", user.is_active);

        // Update
        user.is_active = false;

        println!("User Is Active: {}", user.is_active);

        let build_user = |name: String, email: String| -> User {
            User {
                name,
                email,
                is_active: true,
            }
        };

        let user_built = build_user(String::from("Doe"), String::from("doe@example.com"));

        println!("User Name: {}", user_built.name);
        println!("User Email: {}", user_built.email);
        println!("User Is Active: {}", user_built.is_active);

        let user_3 = User {
            name: String::from("Med"),
            ..user_built
        };

        println!("User Name: {}", user_3.name);
        println!("User Email: {}", user_3.email);
        println!("User Is Active: {}", user_3.is_active);

        // Tuple struct
        #[derive(Debug)]
        struct Color(i32, i32, i32);

        let black = Color(0, 0, 0);

        println!("Color: {:?}", black);
        println!("Red value: {:?}", black.0);
        println!("Green value: {:?}", black.1);
        println!("Blue value: {:?}", black.2);
    }

    #[test]
    fn unit_like_struct() {
        #[derive(Debug)]
        struct AlwaysEqual;

        let a = AlwaysEqual;
        let b = AlwaysEqual;

        println!("a is {:?}, b is {:?}", a, b);
    }

    #[test]
    fn struct_ownership() {
        struct User<'a> {
            active: bool,
            username: &'a str,
            email: &'a str,
            sign_in_count: u64,
        }

        let user1 = User {
            active: true,
            username: "someusername123",
            email: "someone@example.com",
            sign_in_count: 1,
        };

        println!("Username: {}", user1.username);
        println!("Email: {}", user1.email);
        println!("Sign in count: {}", user1.sign_in_count);
        println!("Active: {}", user1.active);
    }

    #[test]
    fn struct_method() {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }

            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }

            fn square(size: u32) -> Rectangle {
                Rectangle {
                    width: size,
                    height: size,
                }
            }
        }

        let rect = Rectangle {
            width: 5,
            height: 10,
        };

        let rect2 = Rectangle {
            width: 3,
            height: 4,
        };

        println!("Rectangle area: {}", rect.area());
        println!("Can rect hold rect2? {}", rect.can_hold(&rect2));

        let square = Rectangle::square(5);

        println!("Square area: {}", square.area());

        // Multiple implementation
        impl Rectangle {
            fn perimeter(&self) -> u32 {
                2 * (self.width + self.height)
            }
        }

        let rect = Rectangle {
            width: 5,
            height: 10,
        };

        println!("Rectangle perimeter: {}", rect.perimeter());
    }
}
