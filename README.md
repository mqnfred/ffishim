# FFIShim

Many common rust types (like `String` for example) cannot be sent over FFI
because their layouts in memory do not match the C ABI (`String` does not have
a null byte at its end.) It makes it hard to use native rust types and call
that code from an FFI.

This crate provides an ffi shim which will expose FFI-compatible data
structures and function wrappers for every native rust type and function.

Here is a quick example of rust code using a native `String` and calling that
from C:

```rust
#[ffishim_use_case]
fn hello(s: String) -> String {
    format!("Hello, {}!", s)
}
```

You should be able to call this from C as follows:

```c
#include "ffishim/header.h"
extern result_t *ffi_hello(char *s);
int main() {
	char *ffi = malloc(sizeof(char) * 4);
	ffi[0] = "f";
	ffi[1] = "f";
	ffi[2] = "i";
	ffi[3] = "\0";

	result_t *res = ffi_hello(ffi)
	if (res->message != NULL) {
		printf("error: %s\n", res->message);
		free(res->message);
	} else {
		printf("%s\n", res->payload);
		free(res->payload);
	}
	free(res);
}
```

Since some type transformations might fail, the functions always return a
`result_t` type, which contains a pointer to an error message and a payload. In
case of an error, the payload is nil and the message contains the error string.

## More examples

You can find more examples of the shim's behavior by looking at the
[`tests`][1] folder. The structure of the tests are as follow:

 - `src/lib.rs`: the rust library to expose
 - `Cargo.toml`: manifest of the rust library
 - `main.c`: the C code that uses this rust library
 - `expected_output`: contains the output expected from running the C program

Every test crate is a stand-alone app, it will provide you with the options
required to build the library (cdylib) etc.

## TODO/Limitations

This crate is still in beta. It is not fit for production use yet.

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

[1]: tests/
