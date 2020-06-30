extern crate proc_macro;
mod once;

use ::darling::FromDeriveInput;

/// Mark a structure for use across the ffi boundary.
///
/// This procedural macro will generate a sister structure called `FFIMyStruct`. It contains the
/// same fields in the same order, except in that they are represented as ffi compatible types: a
/// [`CString`][1] instead of the original struct's [`String`][2] for example.
///
/// You can find the list of types supported by ffishim [here][7]. You can only use those types in
/// fields of structures that derive `FFIShim`, as well as other structures that themselves derive
/// `FFIShim`.
///
/// An implementation of `from` and `try_into` is provided for `FFIMyStruct`, which can be used
/// for back-and-forth "translation." These translations are executed whenever `MyStruct` is passed
/// to or returned from an [`ffishim_use_case`][3], the macro which generates stubs around our
/// functions. You should never have to deal with `FFIMyStruct` yourself at any time.
///
/// # C ABI Violation: embedded structs
///
/// Aside from built-in types, nothing is directly embedded in the struct, it is dereferenced
/// behind a pointer. This is because this project is originally written to work with Dart's [new
/// alpha ffi][4], and this version alpha [does not support embeded structures yet][5].
///
/// This is also a reason this ffi shim currently performs many more dynamic memory allocations
/// than would be necessary provided we could use straight-forward ffi structure embeds.
///
/// # Opaque feature
///
/// If you want to embed structures which have no ffi shim, you can mark them as `opaque` like
/// this:
///
/// ```ignore
/// #[derive(FFIShim)]
/// pub struct MyStruct {
///     #[ffishim(opaque)]
///     not_ffi_compatible: ::std::collections::HashMap<i64, i64>,
/// }
/// ```
///
/// Here, the sister structure still features the field in rust, but it is stored in its native
/// rust form. This should make it virtually unusable from across the ffi boundary, but we will not
/// loose it.
///
/// In this case, the `not_ffi_compatible` field will be carried by the corresponding ffi shim
/// `FFIMyStruct`, but it will not be formatted in any way, and will not be consumable from across
/// the ffi boundary.
///
/// If you have many of such fields in the structure, you might be interested in making the whole
/// structure opaque, in which case none of its fields are exposed:
///
/// ```ignore
/// #[derive(FFIShim)]
/// #[ffishim(opaque)]
/// pub struct MyStruct {
///     not_ffi_compatible: ::std::collections::HashMap<i64, i64>,
/// }
/// ```
///
/// [1]: https://doc.rust-lang.org/std/ffi/struct.CString.html
/// [2]: https://doc.rust-lang.org/std/string/struct.String.html
/// [3]: attr.ffishim_use_case.html
/// [4]: https://dart.dev/guides/libraries/c-interop
/// [5]: https://github.com/dart-lang/sdk/issues/37271
/// [6]: https://github.com/dart-lang/sdk/issues/41062
/// [7]: https://docs.rs/ffishim/0.1.0/ffishim/types/index.html
#[proc_macro_derive(FFIShim, attributes(ffishim))]
pub fn derive_ffishim(stream: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
    let derive_input = ::syn::parse_macro_input!(stream as ::syn::DeriveInput);

    let shim_data = ::ffishim::Data::from_derive_input(&derive_input).unwrap();
    let shim_from = ::ffishim::From::from(&shim_data);
    let shim_try_into = ::ffishim::TryInto::from(&shim_data);
    let shim_news = ::ffishim::News::from(&shim_data);
    let shim_free = ::ffishim::Free::from(&shim_data);
    let shim_allocator_setting = unsafe {
        crate::once::defined_once("shim_allocator_setting", ::ffishim::shim_allocator_setting())
    };

    (::quote::quote! {
        #shim_data
        #shim_from
        #shim_try_into
        #shim_news
        #shim_free
        #shim_allocator_setting
    }).into()
}

/// Mark a function for use across the ffi boundary.
///
/// This procedural macro will generate a stub called `ffi_my_function`. This function features the
/// same arguments as `my_function`, which it wraps. It will convert any arguments from the
/// [`FFIMyStruct`][1] version passed by the caller to the native `MyStruct` version. If there is
/// any return value, it will convert it from its native version back into the ffi one.
///
/// You should never have to call or manipulate `ffi_my_function` from the rust side. It will
/// systematically return our C-ABI equivalent of a `Result`, even if the original `my_function`
/// does not return `Result` itself. This is because the ffi->native type translation can fail, and
/// we need an elegant way to report that (panicking in such a setup is often inacceptable.)
///
/// Take a look at which basic/built-in types you can use [here][4]. You can also use any structure
/// which derive the [`FFIShim`][1] procedural macro.
///
/// # C ABI Violation: passing structs by value
///
/// Any structure passed by value in `my_function` will instead be hidden behind a pointer.
/// This is because [Dart's alpha ffi][2] [does not support passing structures by value][3], and
/// this project was originally written to work with it.
///
/// # Performance implications
///
/// By "type conversions", we mean calling the given structures' `from` and `try_into`
/// implementations. For scalar types for example, that's a simple `mov`. For `String`s, it most
/// likely means a malloc and a memcpy. For complex structures, it is a recursive conversion of all
/// types and sub-types.
///
/// These conversions can quickly become non-trivial, which is why we encourage the user to try to
/// reduce amount of calls and data passed through those calls to the minimum required. If you
/// don't really need a field across the ffi barrier, consider making it [opaque][4].
///
/// [1]: derive.FFIShim.html
/// [2]: https://dart.dev/guides/libraries/c-interop
/// [3]: https://github.com/dart-lang/sdk/issues/41062
/// [4]: https://docs.rs/ffishim/0.1.0/ffishim/types/index.html
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
