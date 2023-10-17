fn main() {
    if let Ok(val) = std::env::var("TEST_FOO") {
        if let Ok(e) = val.parse::<u64>() {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            if timestamp >= e && timestamp < e + 10 {
                // Valid value, instruct Cargo to use it
                println!("cargo:rustc-cfg=feature=\"test_foo\"");
            }
        }
    }
}
