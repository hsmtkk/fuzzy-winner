pub fn required_string(key: &str) -> String {
    let msg = format!("{} environment variable must be defined", key);
    std::env::var(key).expect(&msg)
}
