#![feature(vec_into_raw_parts)]
#[macro_use]
extern crate darling;
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
    ffi_name: ::syn::Ident,
    ffi_args: Vec<::syn::FnArg>,
    ffi_output: ::syn::ReturnType,
    call_expr: ::syn::Expr,
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
    orig_name: ::syn::Ident,
    ffi_name: ::syn::Ident,
    receiver: ::syn::Expr,
    init_expr: ::syn::Expr,
}
mod try_into;

pub mod library;
pub mod types;

/// Macros to extend the `syn` tree and other useful shortcuts.
#[macro_use]
mod helpers;
