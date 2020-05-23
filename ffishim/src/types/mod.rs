//! Definition of accepted types and their behavior.
//!
//! Here is a list of the available complex types and their FFI equivalents.
//!
//! If a type deriving the ffi shim contains references to types that are not listed below, they
//! will by default be considered `Foreign` types. It will be assumed that `Foreign` types
//! derive the ffi shim themselves, and will be manipulated as such. See the `Foreign` behavior for
//! more details.
//!
//! |Complex types|FFI equivalent|
//! |---|---|
//! |`String`|`*mut u8`|
//! |`Option<T>`|`*mut T`|
//! |`Vec<T>`|`ffishim::library::FFIVec<T>`|
//! |`Result<T, E>`|`ffishim::library::FFIResult<T>`|
//! |`chrono::Duration`|`libc::c_long`|
//!
//! And below a list of all the scalar types handled (see the `Scalars` behavior for more details.)
//!
//! |Scalar types|FFI equivalent|
//! |---|---|
//! |`bool`|`libc::c_char`|
//! |`char`|`libc::c_uint`|
//! |`f32`|`libc::c_float`|
//! |`f64`|`libc::c_double`|
//! |`u8`|`libc::c_char`|
//! |`u16`|`libc::c_ushort`|
//! |`u32`|`libc::c_uint`|
//! |`u64`|`libc::c_ulong`|
//! |`usize`|`libc::size_t`|
//! |`i8`|`libc::c_schar`|
//! |`i16`|`libc::c_short`|
//! |`i32`|`libc::c_int`|
//! |`i64`|`libc::c_long`|
//! |`isize`|`libc::ssize_t`|
//!
//! If you want to add a type behavior, just:
//!
//!  - Create a new struct in a sub-module
//!  - Implement the `Behavior` trait
//!  - Add the struct in the private `BEHAVIORS` vector

use ::once_cell::sync::Lazy;

/// The behavior of a `Type` as needed to generate the ffi shim.
pub trait Behavior: Sync + Send {
    /// Returns `true` if this behavior is fit to handle the given `sty`.
    fn is(&self, sty: &::syn::Type) -> bool;

    /// Transforms the given `Type` into its shim equivalent. For example:
    ///  - `String` -> `*mut libc::c_char`
    ///  - `u32` -> `libc::c_uint`
    ///  - ...
    fn fold(&self, sty: ::syn::Type) -> ::syn::Type;

    /// Returns an `Expr`ession that mutates the shim type into the original one.
    fn try_into(&self, sty: &::syn::Type, name: ::syn::Expr) -> ::syn::Expr;

    /// Returns an `Expr`ession that mutates the original type into its shim equivalent.
    fn from(&self, sty: &::syn::Type, name: ::syn::Expr) -> ::syn::Expr;

    /// Returns an `Expr`ession that frees an expression of the given type.
    fn free(&self, sty: &::syn::Type, name: ::syn::Expr) -> ::std::option::Option<::syn::Expr>;
}

/// Switch over a given `Type` and return the associated `Behavior`.
///
/// This is an open-ended, c-style switch: if two different type behaviors' `is` method return
/// `true`, the first one in the list will win. You can order the type behaviors in the
/// `BEHAVIORS` vector.
pub fn switch<'a, 'b>(sty: &'a ::syn::Type) -> &'b Box<dyn Behavior> {
    // TODO: give more context about which type we do not find the behavior of
    BEHAVIORS.iter().find(|tyb| tyb.is(sty)).expect("cannot find behavior for given type")
}

/// List of available, instantiated `Behavior`s.
static BEHAVIORS: Lazy<Vec<Box<dyn Behavior>>> = Lazy::new(|| {
    vec![
        // End-types
        Box::new(Scalars),
        Box::new(Bool),
        Box::new(Char),
        Box::new(Duration),
        Box::new(String),

        // Parameterized types
        Box::new(Option),
        Box::new(Result),
        Box::new(Vector),
        Box::new(Reference),

        // Foreign/custom types implementing an ffi shim
        Box::new(Foreign),
    ]
});

// End-types
mod scalars;
pub use scalars::Behavior as Scalars;
mod bool;
pub use self::bool::Behavior as Bool;
mod char;
pub use self::char::Behavior as Char;
mod duration;
pub use duration::Behavior as Duration;
mod string;
pub use string::Behavior as String;

// Parameterized types
mod option;
pub use option::Behavior as Option;
mod result;
pub use result::Behavior as Result;
mod vec;
pub use vec::Behavior as Vector;
mod reference;
pub use reference::Behavior as Reference;

// Foreign/custom types implementing an ffi shim
mod foreign;
pub use foreign::Behavior as Foreign;
