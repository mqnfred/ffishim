# FFIShim

Generate a thin FFI layer for your rust API using procedural macros.

## TODO

### Implementation

 - `from` should become `try_from`? (invalid rust string -> CString unsafe)
 - Replace functions if `feature(ffishim)` instead of `ffi_` prefix
 - Reconsider dependence on `::anyhow::Error` explicitly
 - Implement opaque types/fields

### Testing

 - Add a "complete" test/example situation

### Documentation

 - Write a good README
 - Write `ffishim_derive` documentation on how to use macros
