#[cfg(test)]
mod tests {
    #[test]
    fn test_log() {
        log4rs::init_file("../../log4rs.yaml", Default::default()).unwrap();

        log::error!("Hello, error!");
        log::warn!("Hello, warn!");
        log::info!("Hello, info!");
        log::debug!("Hello, debug!");
        log::trace!("Hello, trace!");
    }
}
