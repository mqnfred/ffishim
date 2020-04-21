//! Types to be used by the FFI shim in the target programs.
//!
//! The `library` contains all recurrent types that do not need to be generated, for example
//! arrays. The generated shim code will refer to library types whenever necessary.
//!
//! We re-export rust's `libc` here to prevent users from having to declare libc as a direct
//! dependency of their programs.

pub extern crate libc;
extern crate anyhow;
pub use anyhow::Error;

/// A C-compatible array structure to replace rust's `Vec` in the shim layer.
#[repr(C)]
pub struct Array<T> {
    // TODO: phantom market lifetime??
    // TODO: usize ds la struct
    pub ptr: *mut T,
    pub len: usize,
    pub cap: usize,
}
mod array;

/// A C-compatible result structure to replace rust's `Result` in the shim layer.
///
///  - A successful result is defined by a null `message`.
///  - A failure result is defined by a non-null `message`.
///
/// The Result can be considered successful even though the payload is nil, for example in the
/// case where the method returns nothing.
#[repr(C)]
pub struct Result<T> {
    pub error: *mut ::libc::c_char,
    pub payload: *mut T,
}
mod result;
