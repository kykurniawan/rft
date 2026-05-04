#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn new_hash_map() {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        scores.insert(String::from("Red"), 90);

        println!("The hash map is: {:?}", scores);

        // Accessing value
        let score = scores.get("Blue");
        match score {
            Some(s) => println!("The score for Blue is: {}", s),
            None => println!("No score for Blue"),
        }
    }

    #[test]
    fn iterating_hash_map() {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        scores.insert(String::from("Red"), 90);

        for (key, value) in &scores {
            println!("The key is {} and the value is {}", key, value);
        }
    }

    #[test]
    fn updating_hash_map() {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        println!("The hash map is: {:?}", scores);

        // Overwriting value
        scores.insert(String::from("Blue"), 20);
        println!("The updated hash map is: {:?}", scores);

        // Add if not exists
        scores.entry(String::from("Green")).or_insert(30);
        println!("The updated hash map is: {:?}", scores);

        // Updating based on old value
        let text = "Hello world! This is a Hello and a world!";
        let mut text_map = HashMap::new();

        for word in text.split_whitespace() {
            let count = text_map.entry(word.to_string()).or_insert(0);
            *count += 1;
        }

        println!("The text map is: {:?}", text_map);
    }
}
