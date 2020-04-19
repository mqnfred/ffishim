# FFIShim

Generate a thin FFI layer for your rust API using procedural macros.

## TODO

### Implementation

 - [ ] `try_into` struct transformation
 - [ ] `Vec` type behavior
 - [ ] `Result` type behavior (and use it in `function.rs`)
 - [ ] Foreign type behavior

 - [ ] Should `from` become `try_from`? (invalid rust string -> CString unsafe)
 - [ ] Reconsider ffishim's `Error` type (anyhow? trait? ...)
 - [ ] Figure out `Array` initialization from C caller

### Testing

 - [ ] Come up with small dedicated macros to ease consumption of `outcome_t`s
 - [ ] Add a "complete" test/example situation

### Documentation

 - [ ] Write a good README
 - [ ] Add documentation to main `ffishim/src/lib.rs` types
