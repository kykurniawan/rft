#[cfg(test)]
mod tests {
    #[test]
    fn new_vector() {
        let mut numbers: Vec<i32> = Vec::new();

        numbers.push(1);
        numbers.push(2);
        numbers.push(3);

        println!("The vector is: {:?}", numbers);

        // Accessing element by index
        println!("The second element is: {}", numbers[1]);

        // Accessing element with get method
        let first: Option<&i32> = numbers.get(0);
        match first {
            Some(n) => println!("The first element is: {}", n),
            None => println!("No first element"),
        }

        // Iterating over vector
        for number in &numbers {
            println!("{}", number);
        }

        // Iterating over vector and update the value
        for number in &mut numbers {
            *number *= 2;
        }

        println!("The vector after update is: {:?}", numbers);
    }

    #[test]
    fn vector_using_enum() {
        #[derive(Debug)]
        enum SpreadhseetCell {
            Int(i32),
            Float(f32),
            Text(String),
        }

        let row = vec![
            SpreadhseetCell::Int(1),
            SpreadhseetCell::Float(2.0),
            SpreadhseetCell::Text("Hello".to_string()),
        ];

        for cell in row {
            match cell {
                SpreadhseetCell::Int(n) => println!("The cell is an integer: {}", n),
                SpreadhseetCell::Float(n) => println!("The cell is a float: {}", n),
                SpreadhseetCell::Text(s) => println!("The cell is a text: {}", s),
            }
        }
    }
}
