#[macro_use]
extern crate darling;
#[macro_use]
extern crate lazy_static;
extern crate proc_macro2;

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

#[derive(Debug, FromField)]
#[darling(attributes(ffishim))]
pub struct Field {
    ident: Option<::syn::Ident>,
    vis: ::syn::Visibility,
    ty: ::syn::Type,
}
mod field;

#[derive(Debug, FromVariant)]
#[darling(attributes(ffishim))]
pub struct Variant {
    #[darling(default)]
    constructor: Option<::syn::Path>,

    ident: ::syn::Ident,
    fields: ::darling::ast::Fields<Field>,
}
mod variant;

#[derive(Debug)]
pub struct Function {
    ident: ::syn::Ident,
    args: Vec<::syn::FnArg>,
    ret: ::syn::ReturnType,
}
mod function;

pub struct From {
    orig_name: ::syn::Ident,
    ffi_name: ::syn::Ident,
    receiver: ::syn::Expr,
    init_expr: ::syn::Expr,
}
mod from;

pub struct TryInto {
    
}
mod try_into;

/*
#![feature(vec_into_raw_parts)]

use ::syn::*;

/// Folds the provided function into a ffishim wrapper.
struct Folder {
    convert_exprs: Vec<Expr>,
    call_expr: Option<Expr>,
}
mod folder;

/// Spawns an implementation of `from` from the original struct to the ffishim one.
struct From {
    name: Option<Ident>,
    receiver: Option<Ident>,
}
mod from;

/// Spawns an implementation of `try_into` from the ffishim struct to the original one.
struct TryInto;
mod try_into;

/// A c-compatible array structure to replace rust's `Vec` in the shim layer.
#[repr(C)]
pub struct Array<T> {
    pub ptr: *mut T,
    pub len: u64,
    pub cap: u64,
}
/// A c-compatible outcome structure to replace rust's `Result` in the shim layer.
#[repr(C)]
pub struct Outcome<T> {
    pub errorcode: i64,
    pub message: *mut ::std::os::raw::c_char,
    pub payload: *mut T,
}
mod libffishim;
*/

/// The behavior of a `Type` as needed by this crate.
///
/// `is` determines if this behavior applies to the given `Type`.
/// `fold` folds the original `Type` into the ffishim `Type`.
/// `try_into` generates a `Expr` that transforms an object from the original into its ffishim.
/// `from` generates a `Expr` that transforms an object from its ffishim into the original.
trait TypeBehavior: Sync {
    fn is(&self, sty: &::syn::Type) -> bool;
    fn fold(&self, sty: ::syn::Type) -> ::syn::Type;
    fn try_into(&self, name: ::syn::Expr) -> ::syn::Expr;
    fn from(&self, name: ::syn::Expr) -> ::syn::Expr;
}
/// Switch over a given `Type` and return the associated `Type` behavior.
///
/// This is an open-ended, c-style switch: if two different type behaviors' `is` method returns
/// true, the first one in the list will win. You can order the type behaviors in the
/// `types::BEHAVIORS` vector.
fn switch<'a, 'b>(sty: &'a ::syn::Type) -> &'b Box<dyn TypeBehavior> {
    // TODO: give more context about which type we do not find the behavior of
    types::BEHAVIORS.iter().find(|tyb| tyb.is(sty)).expect("cannot find behavior for given type")
}
mod types;

/// Macros to extend the `syn` tree and other useful shortcuts.
#[macro_use] mod helpers;
