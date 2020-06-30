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
#[darling(attributes(ffishim), map = "Data::validate")]
pub struct Data {
    /// Specify a constructor to construct the original data with when given the ffi form.
    ///
    /// Enumerations cannot expose constructors at the data-structure level, and must instead
    /// expose them on a per-variant basis.
    #[darling(default)]
    pub constructor: Option<::syn::Path>,
    /// The ffi type will not expose any fields of this data structure, making it opaque.
    ///
    /// This is useful in case a data structure you have must be shared through the ffi, but it
    /// cannot be represented/modeled in the other language, or it should not.
    #[darling(default)]
    pub opaque: bool,

    /// The name of the data structure.
    pub ident: ::syn::Ident,
    /// The list of variants (for an enum) or fields (for a struct.)
    pub data: ::darling::ast::Data<Variant, Field>,
}
mod data;

/// A FFIShim field as loaded by darling.
///
/// This structure represents a field as ingested by ffishim. Other structures iterate on sets of
/// `Field`s to generate code.
#[derive(Debug, FromField)]
#[darling(attributes(ffishim))]
pub struct Field {
    /// This field will not be exposed as such and instead be replaced by a pointer to the value.
    ///
    /// This is useful in case a type you want to expose through the ffi holds one field that
    /// cannot be represented/modeled in the other language, or it should not.
    #[darling(default)]
    pub opaque: bool,

    /// Name of the field (empty for tuple structs.)
    pub ident: Option<::syn::Ident>,
    /// Visibility of the given field. All ffi fields are public by default.
    pub vis: ::syn::Visibility,
    /// Type of the original field.
    pub ty: ::syn::Type,
}
mod field;

/// A FFIShim variant as loaded by darling.
///
/// This structure represents an enum variant as ingested by ffishim. Other structures iterate on
/// sets of `Variant`s and their internal sets of `Field`s to generate code.
#[derive(Debug, FromVariant)]
#[darling(attributes(ffishim))]
pub struct Variant {
    /// For enumerations, constructor is controlled on a per-variant basis.
    #[darling(default)]
    pub constructor: Option<::syn::Path>,

    /// Name of the variant.
    pub ident: ::syn::Ident,
    /// Fields of the variant (empty for unit variants.)
    pub fields: ::darling::ast::Fields<Field>,
}
mod variant;

/// Derived from an `ItemFn` to generate its equivalent wrapper function.
///
/// The `ToTokens` implementation of this structure generates the code for the wrapper around the
/// API's functions. This API performs all structure transformation required to obtain clean types
/// in the rust code and return the FFI types back.
#[derive(Debug)]
pub struct Function {
    pub ffi_name: ::syn::Ident,
    pub ffi_args: Vec<::syn::FnArg>,
    pub ffi_output: ::syn::ReturnType,
    pub call_expr: ::syn::Expr,
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
pub struct News(Vec<::syn::ItemFn>);
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
pub use helpers::shim_allocator_setting;
