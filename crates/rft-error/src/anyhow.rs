#[cfg(test)]
mod test {
    use anyhow::Result;

    fn maybe_error(name: &str) -> Result<bool> {
        if name.len() < 3 {
            return Err(anyhow::anyhow!("Name too short"));
        }

        if name.len() > 10 {
            return Err(anyhow::anyhow!("Name too long"));
        }

        Ok(true)
    }

    #[test]
    fn test_anyhow() {
        let too_short = maybe_error("a");
        assert!(too_short.is_err());

        let too_long = maybe_error("abcdefghijklmnopqrstuvwxyz");
        assert!(too_long.is_err());

        let ok = maybe_error("abc");
        assert!(ok.is_ok());
    }
}
