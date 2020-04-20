# FFIShim

Generate a thin FFI layer for your rust API using procedural macros.

## TODO

### Implementation

 - [x] `try_into` struct transformation
 - [ ] `Vec` type behavior
 - [x] `Result` type behavior (and use it in `function.rs`)
 - [x] `Foreign` type behavior
 - [ ] `free_*` method generation for structs/enums

 - [ ] Should `from` become `try_from`? (invalid rust string -> CString unsafe)
 - [ ] Reconsider ffishim's `Error` type (anyhow? trait? ...)
 - [ ] Figure out `Array` initialization from C caller
 - [ ] Replace functions if `feature(ffishim)` instead of `ffi_` prefix

### Testing

 - [ ] Come up with small dedicated macros to ease consumption of `outcome_t`s
 - [ ] Add a "complete" test/example situation

### Documentation

 - [ ] Write a good README
 - [ ] Add documentation to main `ffishim/src/lib.rs` types
