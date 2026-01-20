Migration notes

- Replaced raw pointers with slices
- Converted error codes to Result<T, E>
- Used #[repr(C)] for ABI compatibility
- Unsafe blocks are minimized in transpiled Rust
