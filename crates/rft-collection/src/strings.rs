#[cfg(test)]
mod tests {
    #[test]
    fn new_string() {
        let s = String::new();
        println!("The string is: {}", s);

        let data = "Hello, world!";
        let data_str = data.to_string();
        println!("The string is: {}", data_str);

        // From
        let data = String::from("Hello, world!");
        println!("The string is: {}", data);
    }

    #[test]
    fn updating_string() {
        // Push string
        let mut s = String::from("Hello, world!");
        s.push_str(", Rust!");
        println!("The updated string is: {}", s);

        // Push single character
        let mut s = String::from("Hello");
        s.push('!');
        println!("The updated string is: {}", s);

        // Concatenate string
        let s1 = String::from("Hello");
        let s2 = String::from(", world!");
        let s3 = s1 + &s2;
        println!("The concatenated string is: {}", s3);

        // Concatenate string with format
        let s1 = String::from("Hello");
        let s2 = String::from(", world!");
        let s3 = format!("{}---{}", s1, s2);
        println!("The formatted string is: {}", s3);
    }

    #[test]
    fn slicing_string() {
        let s = String::from("Hello, world!");

        // Slice string
        let s1 = &s[0..5];
        println!("The sliced string is: {}", s1);

        // Slice string with range
        let s1 = &s[0..2];
        println!("The sliced string is: {}", s1);
    }

    #[test]
    fn iterating_string() {
        let s = String::from("Hello");

        // Iterating over string
        for c in s.chars() {
            println!("{}", c);
        }

        // Iterating bytes
        for b in s.bytes() {
            println!("{}", b);
        }
    }
}
