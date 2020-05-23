# FFIShim

Generate a thin FFI layer for your rust API using procedural macros.

## TODO

### Bugs

 - The scalars type to libc types should either refer only libc types or none
 - Name conflicts arise across crates in .so: for example the function name
   `free_config` if there is a `Config` struct in 2 crates. Need name mangling

### Features

 - `from` should become `try_from`? (invalid rust string -> CString unsafe)
 - Replace functions if `feature(ffishim)` instead of `ffi_` prefix
 - Reconsider dependence on `::anyhow::Error` explicitly
 - Type behavior for chrono dates

### Testing

 - Add a "complete" test/example situation
 - Design benchmarking strategy and framework
 - Add test of enums

### Documentation

 - Write README.md introduction on what this crate does
 - Write `ffishim_derive` documentation on how to use macros
