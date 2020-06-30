# FFIShim

Many common rust types (like `String` for example) cannot be sent over FFI
because their layouts in memory do not match the C ABI (`String` does not have
a null byte at its end.) It makes it hard to use native rust types and call
that code from FFI.

This crate provides a shim which will expose FFI-compatible data structures and
function wrappers for native rust types and functions.

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

This will print:

```
Hello, ffi!
```

Since some type transformations might fail, the functions always return a
`result_t` type, which contains a pointer to an error message and a payload. In
case of an error, the payload is nil and the message contains the error string.

## More examples

You can find more examples of the shim's behavior by looking at the
[`tests`][1] folder. The structure of the tests is as follow:

 - `src/lib.rs`: the rust library to expose
 - `Cargo.toml`: manifest of the rust library
 - `main.c`: the C code that uses this rust library
 - `expected_output`: contains the output expected from running the C program

Every test crate is a stand-alone app leveraging the ffishim library. They will
shed light on how to setup the library and use it.

## C ABI Disclaimer

This crate does not currently generate C ABI-compatible bindings. This is
because it has been designed to be used together with [dustr][2], which
generates Dart bindings on top of this ffi shim.

Because dart ffi support is still in alpha, it cannot quite consume the C ABI
just yet. For example, [it does not support nested structs][3], and [structures
cannot be passed by value to functions][4].

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
[2]: https://github.com/mqnfred/dustr
[3]: https://github.com/dart-lang/sdk/issues/37271
[4]: https://github.com/dart-lang/sdk/issues/41062
