#[cfg(test)]
mod tests {
    #[test]
    fn immutable_variable() {
        let name = "Alice";
        let age = 30;

        println!("Hello, {}! You are {} years old.", name, age);
    }

    #[test]
    fn mutable_variable() {
        let mut name = "Alice";
        let mut age = 30;

        println!("Hello, {}! You are {} years old.", name, age);

        name = "Bob";
        age = 31;

        println!("Hello, {}! You are {} years old.", name, age);
    }

    #[test]
    fn shadowing() {
        let name = "Alice";
        let age = 30;

        println!("Hello, {}! You are {} years old.", name, age);

        let name = "Bob";
        let age = 31;

        println!("Hello, {}! You are {} years old.", name, age);
    }

    #[test]
    fn constant() {
        const NAME: &str = "Alice";
        const AGE: u32 = 30;

        println!("Hello, {}! You are {} years old.", NAME, AGE);
    }
}
