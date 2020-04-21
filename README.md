# FFIShim

Generate a thin FFI layer for your rust API using procedural macros.

## TODO

### Implementation

 - Should `from` become `try_from`? (invalid rust string -> CString unsafe)
 - Reconsider ffishim's `Error` type (anyhow? trait? ...)
 - Figure out `Array` initialization from C caller
 - Replace functions if `feature(ffishim)` instead of `ffi_` prefix
 - Error out if reference type encountered in API

### Testing

 - Add a "complete" test/example situation

### Documentation

 - Write a good README
