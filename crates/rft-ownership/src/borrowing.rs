#[cfg(test)]
mod tests {
    #[test]
    fn borrowing() {
        let s = String::from("Hello");
        let len = calculate_length(&s);
        println!("The length of '{}' is {}.", s, len);
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    #[test]
    fn mutable_reference() {
        let mut s = String::from("Hello");

        change(&mut s);

        println!("{}", s)
    }

    fn change(s: &mut String) {
        s.push_str(", world!");
    }
}
