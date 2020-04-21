//! The backend for the `ffishim_derive` procedural macro.
//!
//! This crate provides:
//!
//!  - The C types used by the "ffi shim" layer such as `FFIVec` and `FFIResult` for example.
//!  - The logic for generating a "ffi shim" layer around your API. See the `ffishim_derive`
//!    macro for more documentation on that.
//!
//! You most likely should not use this API directly, and rely on the `ffishim_derive` procedural
//! macro for all your ffi shim needs.

#![feature(vec_into_raw_parts)]
#[macro_use]
extern crate darling;
extern crate proc_macro2;

/// Entry point of the ffishim crate.
///
/// This data is generated from a `DeriveInput` by the darling crate. Its `IntoTokens`
/// implementations generates the `FFIName` equivalent data structure, where `Name` is the name of
/// the original data structure.
///
/// It is also consumed by multiple other objects in this crate for generation of other critical
/// logic (`From`, `TryInto` ...)
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(ffishim), map = "Data::initialize")]
pub struct Data {
    #[darling(default)]
    constructor: Option<::syn::Path>,
    #[darling(default)]
    opaque: bool,

    ident: ::syn::Ident,
    data: ::darling::ast::Data<Variant, Field>,
}
mod data;

/// A FFIShim field as loaded by darling.
///
/// This structure represents a field as ingested by ffishim. Other structures iterate on sets of
/// `Field`s to generate code.
#[derive(Debug, FromField)]
#[darling(attributes(ffishim))]
pub struct Field {
    ident: Option<::syn::Ident>,
    vis: ::syn::Visibility,
    ty: ::syn::Type,
}
mod field;

/// A FFIShim variant as loaded by darling.
///
/// This structure represents an enum variant as ingested by ffishim. Other structures iterate on
/// sets of `Variant`s and their internal sets of `Field`s to generate code.
#[derive(Debug, FromVariant)]
#[darling(attributes(ffishim))]
pub struct Variant {
    #[darling(default)]
    constructor: Option<::syn::Path>,

    ident: ::syn::Ident,
    fields: ::darling::ast::Fields<Field>,
}
mod variant;

/// Derived from an `ItemFn` to generate its equivalent wrapper function.
///
/// The `ToTokens` implementation of this structure generates the code for the wrapper around the
/// API's functions. This API performs all structure transformation required to obtain clean types
/// in the rust code and return the FFI types back.
#[derive(Debug)]
pub struct Function {
    ffi_name: ::syn::Ident,
    ffi_args: Vec<::syn::FnArg>,
    ffi_output: ::syn::ReturnType,
    call_expr: ::syn::Expr,
}
mod function;

/// Derived from `Data` to generate the `FFIName::from(Name)` conversion.
pub struct From {
    orig_name: ::syn::Ident,
    ffi_name: ::syn::Ident,
    receiver: ::syn::Expr,
    init_expr: ::syn::Expr,
}
mod from;

/// Derived from `Data` to generate the `FFIName::try_into(Name)` conversion.
pub struct TryInto {
    orig_name: ::syn::Ident,
    ffi_name: ::syn::Ident,
    receiver: ::syn::Expr,
    init_expr: ::syn::Expr,
}
mod try_into;

/// Derived from `Data` to generate the `new_name` function.
///
/// This function can be called from the caller code to initialize a ffishim data structure
/// elegantly, without having to resort to its own malloc.
pub struct New {
    new_funcs: Vec<::syn::ItemFn>,
}
mod new;

/// Derived from `Data` to generate the `free_name` function.
///
/// This function takes care of freeing the provided data structure and all its hierarchy. This
/// prevents the caller from having to understand the specifics.
pub struct Free {
    func_name: ::syn::Ident,
    receiver: ::syn::Expr,
    ffi_type: ::syn::Type,
    free_expr: ::syn::Expr,
}
mod free;

pub mod library;
pub mod types;

/// Macros to extend the `syn` tree and other useful shortcuts.
#[macro_use]
mod helpers;
