extern crate proc_macro;

use ::darling::FromDeriveInput;

#[proc_macro_derive(FFIShim, attributes(ffishim))]
pub fn derive_ffishim(stream: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
    let di = ::syn::parse_macro_input!(stream as ::syn::DeriveInput);
    let shim = ::ffishim::Data::from_derive_input(&di).unwrap();
    let from = ::ffishim::From::from(&shim);
    //let try_into = ::ffishim::TryInto::from(&shim);
    (::quote::quote! { #shim #from }).into()
}

#[proc_macro_attribute]
pub fn ffishim(
    _: ::proc_macro::TokenStream,
    stream: ::proc_macro::TokenStream,
) -> ::proc_macro::TokenStream {
    let original: ::proc_macro2::TokenStream = stream.clone().into();
    let ifn = ::syn::parse_macro_input!(stream as ::syn::ItemFn);
    let shim = ::ffishim::Function::from_item_fn(&ifn).unwrap();
    (::quote::quote! { #original #shim }).into()
}
