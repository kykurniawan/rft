use std::fs::File;
use std::io::Read;

mod panic;
mod result;

fn read_api_key_from_file() -> Result<String, std::io::Error> {
    let mut api_key_file = File::open("api_key.txt")?;
    let mut api_key = String::new();
    api_key_file.read_to_string(&mut api_key)?;
    Ok(api_key)
}

fn main() {
    match read_api_key_from_file() {
        Ok(api_key) => println!("The API key is: {}", api_key),
        Err(error) => println!("Error reading the API key: {:?}", error),
    }
}
