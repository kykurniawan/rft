#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn calling_panic() {
        panic!("This is a panic");
    }

    #[test]
    #[should_panic]
    fn panic_from_library() {
        let v = vec![1, 2, 3];
        v[4];
    }
}
