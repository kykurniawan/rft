#[cfg(test)]
mod tests {
    #[test]
    fn define_enum() {
        #[derive(Debug)]
        enum IpAddrKind {
            V4,
            V6,
        }

        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        println!("four is {:?}, six is {:?}", four, six);
    }

    #[test]
    fn enum_values() {
        #[derive(Debug)]
        enum IpAddrKind {
            V4(String),
            V6(String),
        }

        let four = IpAddrKind::V4(String::from("127.0.0.1"));
        let six = IpAddrKind::V6(String::from("::1"));

        println!("four is {:?}, six is {:?}", four, six);

        if let IpAddrKind::V4(four) = four {
            println!("four: {}", four);
        }

        if let IpAddrKind::V6(six) = six {
            println!("six: {}", six);
        }
    }

    #[test]
    fn enum_methods() {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        impl Message {
            fn call(&self) {
                // Method implementations go here
                println!("Calling a message");

                if let Message::Quit = self {
                    println!("Received quit message");
                }
                if let Message::Move { x, y } = self {
                    println!("Move message: x: {}, y: {}", x, y);
                }
                if let Message::Write(s) = self {
                    println!("Write message: {}", s);
                }
                if let Message::ChangeColor(r, g, b) = self {
                    println!("ChangeColor message: r: {}, g: {}, b: {}", r, g, b);
                }
            }
        }

        let m = Message::Quit;
        m.call();

        let m = Message::Move { x: 1, y: 2 };
        m.call();

        let m = Message::Write(String::from("hello"));
        m.call();

        let m = Message::ChangeColor(1, 2, 3);
        m.call();
    }

    #[test]
    fn enum_option() {
        let some_number = Some(5);
        let some_char = Some('a');
        let absent_number: Option<i32> = None;

        println!("Some Number: {:?}", some_number);
        println!("Some Char: {:?}", some_char);
        println!("Absent Number: {:?}", absent_number);
    }

    #[test]
    fn enum_match() {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
            Wyoming,
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }

        let value_in_cent = |coin: Coin| -> u32 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        };

        let five_pennies = Coin::Penny;
        let five_nickels = Coin::Nickel;
        let ten_dimes = Coin::Dime;
        let twenty_five_state_alaska = Coin::Quarter(UsState::Alaska);
        let twenty_five_state_wyoming = Coin::Quarter(UsState::Wyoming);
        let twenty_five_state_albama = Coin::Quarter(UsState::Alabama);

        println!("Value in cents: {}", value_in_cent(five_pennies));
        println!("Value in cents: {}", value_in_cent(five_nickels));
        println!("Value in cents: {}", value_in_cent(ten_dimes));
        println!(
            "Value in cents: {}",
            value_in_cent(twenty_five_state_alaska)
        );
        println!(
            "Value in cents: {}",
            value_in_cent(twenty_five_state_wyoming)
        );
        println!(
            "Value in cents: {}",
            value_in_cent(twenty_five_state_albama)
        );
    }

    #[test]
    fn option_t_match_pattern() {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        println!("Five: {:?}", five);
        println!("Six: {:?}", six);
        println!("None: {:?}", none);
    }

    #[test]
    fn if_let() {
        let config_max = Some(3u8);

        if let Some(max) = config_max {
            println!("Max: {}", max);
        } else {
            println!("No max");
        }
    }
}
