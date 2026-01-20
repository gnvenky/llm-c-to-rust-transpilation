pub mod parser;

#[cfg(test)]
mod tests {
    use super::parser::*;
    
    #[test]
    fn test_parse_message() {
        let data = [1u8, 0x00, 0x2A];
        let buf = &data[..];
        let result = parse_message(buf);
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert_eq!(msg.id, 1);
        assert_eq!(msg.value, 42);
    }
}
// LLM-generated Rust
