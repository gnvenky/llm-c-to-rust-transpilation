pub mod parser;

#[cfg(test)]
mod tests {
    use super::parser::*;
    
    #[test]
    fn test_parse_message() {
        let data = [1u8, 0x00, 0x2A];
        let result = parser::parse_message(&data);
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert_eq!(msg.id, 1);
        assert_eq!(msg.value, 42);
    }
}
// idiomatic Rust
