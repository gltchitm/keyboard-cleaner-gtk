pub fn now() -> u128 {
    let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH);
    time.unwrap().as_millis()
}
