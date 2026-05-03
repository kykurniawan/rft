#[cfg(test)]
mod tests {
    #[test]
    fn move_semantics() {
        let x = 5;
        let y = x;
        println!("x: {}, y: {}", x, y);

        // Move string
        let s1 = String::from("Hello");
        println!("s1: {}", s1); // s1 available here
        let s2 = s1; // s2 points to the same string as s1
        println!("s2: {}", s2); // s2 available here, s1 is no longer available
    }

    #[test]
    fn clone_semantics() {
        let s1 = String::from("Hello");
        let s2 = s1.clone();
        println!("s1: {}, s2: {}", s1, s2);
    }

    #[test]
    fn ownership_and_function() {
        let s = String::from("Hello");
        let s_cloned = s.clone();

        takes_ownership(s);

        println!("s_cloned: {}", s_cloned);

        let x = 5;
        makes_copy(x);
    }

    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    }

    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer);
    }
}
