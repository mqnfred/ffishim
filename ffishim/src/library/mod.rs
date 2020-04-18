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
    pub ptr: *mut T,
    pub len: u64,
    pub cap: u64,
}
mod array;

/// A C-compatible outcome structure to replace rust's `Result` in the shim layer.
///
///  - A successful outcome is defined by a 0 `errorcode` and a non-NULL `payload`.
///  - A failure outcome is defined by a non-0 errorcode and a non-NULL `message`.
#[repr(C)]
pub struct Outcome<T> {
    pub error: *mut ::libc::c_char,
    pub payload: *mut T,
}
mod outcome;
