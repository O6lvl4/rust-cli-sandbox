
pub fn hello_world() -> String {
    return "Hello, World!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    # [test]
    fn test_hello_world() {
        assert_eq!("Hello, World!", hello_world());
    }
}