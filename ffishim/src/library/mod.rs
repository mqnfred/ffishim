//! Types to be used by the FFI shim in the target programs.
//!
//! The `library` contains all recurrent types that do not need to be generated, for example
//! the C-compatible "vectors." The generated shim code will refer to library types whenever
//! necessary.
//!
//! We re-export rust's `libc` here to prevent users from having to declare libc as a direct
//! dependency of their programs.

pub extern crate libc;
extern crate anyhow;
pub use anyhow::Error;

/// A C-compatible vector structure to replace rust's `Vec` in the shim layer.
#[repr(C)]
pub struct FFIVec<T> {
    // TODO: phantom market lifetime??
    pub ptr: *mut T,
    pub len: usize,
    pub cap: usize,
}
mod vec;

/// A C-compatible result structure to replace rust's `Result` in the shim layer.
///
///  - A successful result is defined by a null `message`.
///  - A failure result is defined by a non-null `message`.
///
/// The Result can be considered successful even though the payload is nil, for example in the
/// case where the method returns nothing.
#[repr(C)]
pub struct FFIResult<T> {
    pub error: *mut ::libc::c_char,
    pub payload: *mut T,
}
mod result;
