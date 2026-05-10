#[cfg(test)]
mod tests {
    #[test]
    fn test_log() {
        env_logger::init();

        log::error!("Hello, error!");
        log::warn!("Hello, warn!");
        log::info!("Hello, info!");
        log::debug!("Hello, debug!");
        log::trace!("Hello, trace!");
    }
}
