pub fn now() -> u128 {
    let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH);
    return time.unwrap().as_millis();
}
