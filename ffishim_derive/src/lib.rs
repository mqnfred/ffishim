extern crate proc_macro;
mod once;

use ::darling::FromDeriveInput;

#[proc_macro_derive(FFIShim, attributes(ffishim))]
pub fn derive_ffishim(stream: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
    let derive_input = ::syn::parse_macro_input!(stream as ::syn::DeriveInput);

    let shim_data = ::ffishim::Data::from_derive_input(&derive_input).unwrap();
    let shim_from = ::ffishim::From::from(&shim_data);
    let shim_try_into = ::ffishim::TryInto::from(&shim_data);
    let shim_news = ::ffishim::News::from(&shim_data);
    let shim_free = ::ffishim::Free::from(&shim_data);

    (::quote::quote! {
        #shim_data
        #shim_from
        #shim_try_into
        #shim_news
        #shim_free
    }).into()
}

#[proc_macro_attribute]
pub fn ffishim_use_case(
    _: ::proc_macro::TokenStream,
    stream: ::proc_macro::TokenStream,
) -> ::proc_macro::TokenStream {
    let original_function: ::proc_macro2::TokenStream = stream.clone().into();

    let item_fn = ::syn::parse_macro_input!(stream as ::syn::ItemFn);
    let shim_function = ::ffishim::Function::from_item_fn(&item_fn);
    let free_result_function = unsafe {
        crate::once::defined_once("free_result", ::ffishim::library::free_result_function())
    };

    (::quote::quote! {
        #original_function
        #shim_function
        #free_result_function
    }).into()
}
