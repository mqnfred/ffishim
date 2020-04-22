use ::heck::SnakeCase;
use crate::helpers::*;

impl ::quote::ToTokens for crate::News {
    fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
        let new_funcs = &self.0;
        tokens.extend(::quote::quote! {
            #(#new_funcs)*
        });
    }
}

impl<'a> From<&'a crate::Data> for crate::News {
    fn from(data: &'a crate::Data) -> Self {
        Self(
            if data.opaque {
                vec![]
            } else {
                let orig_name = &data.ident;
                let ffi_name = orig_name.clone().prefix("FFI");

                match &data.data {
                    ::darling::ast::Data::Enum(variants) => enum_new_funcs(
                        &orig_name,
                        &ffi_name,
                        variants,
                        ),
                    ::darling::ast::Data::Struct(fields) => vec![struct_new_func(
                        &orig_name,
                        &::syn::parse_quote! { #ffi_name },
                        fields,
                        )],
                }
            }
        )
    }
}

fn enum_new_funcs(
    orig_name: &::syn::Ident,
    ffi_name: &::syn::Ident,
    variants: &Vec<crate::Variant>,
) -> Vec<::syn::ItemFn> {
    variants.iter().map(|v| {
        let variant_name = &v.ident;
        let orig_enum_variant_name = new_ident(&format!("{}{}", orig_name, variant_name));
        let ffi_name = ::syn::parse_quote! { #ffi_name::#variant_name };
        struct_new_func(&orig_enum_variant_name, &ffi_name, &v.fields)
    }).collect()
}

fn struct_new_func(
    orig_name: &::syn::Ident,
    ffi_name: &::syn::Path,
    fields: &::darling::ast::Fields<crate::Field>,
) -> ::syn::ItemFn {
    let func_name = new_ident(&format!("new_{}", orig_name.to_string().to_snake_case()));

    let field_decls: Vec<::syn::FnArg> = fields.iter().enumerate().map(|(idx, field)| {
        let ident = field.ident.clone().unwrap_or_else(|| idx_to_name(idx as u32));
        let ty = crate::types::switch(&field.ty).fold(field.ty.clone());
        ::syn::parse_quote! { #ident: #ty }
    }).collect();

    let field_names: Vec<::syn::Ident> = fields.iter().enumerate().map(|(idx, field)| {
        field.ident.clone().unwrap_or_else(|| idx_to_name(idx as u32))
    }).collect();

    let init_expr: ::syn::Expr = match fields.style {
        ::darling::ast::Style::Tuple => ::syn::parse_quote! { #ffi_name(#(#field_names),*) },
        ::darling::ast::Style::Struct => ::syn::parse_quote! { #ffi_name{#(#field_names),*} },
        ::darling::ast::Style::Unit => ::syn::parse_quote! { #ffi_name },
    };

    ::syn::parse_quote! {
        #[no_mangle]
        pub extern "C" fn #func_name(#(#field_decls),*) -> *mut #ffi_name {
            Box::into_raw(Box::new(#init_expr))
        }
    }
}
