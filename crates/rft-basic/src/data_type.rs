#[cfg(test)]
mod tests {
    #[test]
    fn type_annotation() {
        // When parsing, we need to add a type annotation to the variable
        // because the compiler need more information to know the type of the variable.
        let guess: u32 = "42".parse().expect("Not a number");
        println!("The number is {}", guess);
    }

    #[test]
    fn scalar_types() {
        // Signed integer
        let x: i32 = 42;
        println!("The number is {}", x);

        // Unsigned integer
        let y: u32 = 42;
        println!("The number is {}", y);

        // Architecture-dependent integer
        let z: isize = 42; // signed integer
        println!("The number is {}", z);
        let z: usize = 42; // unsigned integer
        println!("The number is {}", z);

        // Floating point
        let z: f64 = 42.0;
        println!("The number is {}", z);

        // Boolean
        let w: bool = true;
        println!("The boolean is {}", w);

        // Character
        let x: char = 'a'; // char uses single quotes
        println!("The character is {}", x);
    }

    #[test]
    fn compound_types() {
        // Tuple
        let x: (i32, f64, u8) = (42, 3.14, 1);
        println!("The tuple is {:?}", x);

        // Array
        let y: [u8; 3] = [1, 2, 3];
        println!("The array is {:?}", y);
        let y_str: [&'static str; 3] = ["one", "two", "three"];
        println!("The array is {:?}", y_str);
        // array with same value
        let y_same = [3; 5];
        println!("The array is {:?}", y_same);
        // accesing array
        println!("The first element is {}", y[0]);

        // Slice
        let z: &[u8] = &[1, 2, 3];
        println!("The slice is {:?}", z);

        // String
        let s: String = "Hello, world!".to_string();
        println!("The string is {}", s);
    }
}
