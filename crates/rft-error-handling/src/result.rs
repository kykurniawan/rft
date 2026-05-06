#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{self, ErrorKind, Read},
    };

    #[test]
    #[should_panic]
    fn call_function_that_returns_result() {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };

        println!("The file is: {:?}", greeting_file);
    }

    #[test]
    fn matching_on_different_error() {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                _ => panic!("Problem opening the file: {:?}", error),
            },
        };

        println!("The file is: {:?}", greeting_file);
    }

    #[test]
    fn match_alternatives() {
        let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {error:?}");
                })
            } else {
                panic!("Problem opening the file: {error:?}");
            }
        });

        println!("The file is: {:?}", greeting_file);
    }

    #[test]
    #[should_panic]
    fn unwrap() {
        let greeting_file = File::open("hello.txt").unwrap();

        println!("The file is: {:?}", greeting_file);
    }

    #[test]
    #[should_panic]
    fn expect() {
        let greeting_file = File::open("hello.txt").expect("Failed to open the file");

        println!("The file is: {:?}", greeting_file);
    }

    #[test]
    #[should_panic]
    fn propagating_errors() {
        fn read_username_from_file() -> Result<String, io::Error> {
            let username_file_result = File::open("username.txt");

            let mut username_file = match username_file_result {
                Ok(file) => file,
                Err(error) => return Err(error),
            };

            let mut username = String::new();

            match username_file.read_to_string(&mut username) {
                Ok(_) => Ok(username),
                Err(error) => Err(error),
            }
        }

        fn read_username_from_file_with_question_mark() -> Result<String, io::Error> {
            let mut username_file = File::open("hello.txt")?;
            let mut username = String::new();
            username_file.read_to_string(&mut username)?;
            Ok(username)
        }

        fn question_mark_with_chaining() -> Result<String, io::Error> {
            let mut username = String::new();

            File::open("hello.txt")?.read_to_string(&mut username)?;

            Ok(username)
        }

        read_username_from_file().unwrap();
        read_username_from_file_with_question_mark().unwrap();
        question_mark_with_chaining().unwrap();
    }
}
