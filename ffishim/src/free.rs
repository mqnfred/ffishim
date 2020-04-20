use ::heck::SnakeCase;
use crate::helpers::*;

impl ::quote::ToTokens for crate::Free {
    fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
        let func_name = &self.func_name;
        let receiver = &self.receiver;
        let ffi_type = &self.ffi_type;
        let free_expr = &self.free_expr;

        tokens.extend(::quote::quote! {
            #[no_mangle]
            pub extern "C" fn #func_name(#receiver: #ffi_type) {
                if !#receiver.is_null() {
                    let #receiver = *unsafe { Box::from_raw(#receiver) };
                    #free_expr;
                }
            }
        });
    }
}

impl<'a> From<&'a crate::Data> for crate::Free {
    fn from(data: &'a crate::Data) -> Self {
        let ffi_name = data.ident.clone().prefix("FFI");

        let func_name = new_ident(&format!("free_{}", data.ident.to_string().to_snake_case()));
        let receiver = ::syn::parse_quote! { tmp };
        let ffi_type = ::syn::parse_quote! { *mut #ffi_name };
        let free_expr = match &data.data {
            ::darling::ast::Data::Enum(variants) => enum_free_expr(
                &::syn::parse_quote! { #ffi_name },
                &receiver,
                variants,
            ),
            ::darling::ast::Data::Struct(fields) => struct_free_expr(
                Some(&receiver),
                fields,
            ),
        };

        Self{func_name, receiver, ffi_type, free_expr}
    }
}

fn enum_free_expr(
    ffi_name: &::syn::Path,
    receiver: &::syn::Expr,
    variants: &Vec<crate::Variant>,
) -> ::syn::Expr {
    let arms: Vec<::syn::Arm> = variants.iter().map(|v| {
        let variant_name = &v.ident;
        let ffi_variant_fullpath: ::syn::Path = ::syn::parse_quote! { #ffi_name::#variant_name };

        let free_expr = struct_free_expr(None, &v.fields);
        let destructuring: Vec<::syn::Ident> = v.fields.iter().enumerate().map(|(idx, field)| {
            field.ident.clone().unwrap_or(idx_to_name(idx as u32))
        }).collect();

        match v.fields.style {
            ::darling::ast::Style::Tuple => ::syn::parse_quote! {
                #ffi_variant_fullpath(#(#destructuring),*) => #free_expr
            },
            ::darling::ast::Style::Struct => ::syn::parse_quote! {
                #ffi_variant_fullpath{#(#destructuring),*} => #free_expr
            },
            ::darling::ast::Style::Unit => ::syn::parse_quote! { {} },
        }
    }).collect();

    ::syn::parse_quote! {
        match #receiver {
            #(#arms),*
        }
    }
}

fn struct_free_expr(
    receiver: Option<&::syn::Expr>,
    fields: &::darling::ast::Fields<crate::Field>,
) -> ::syn::Expr {
    let field_exprs = fields.iter().enumerate().filter_map(|(idx, field)| {
        field.free(idx, receiver)
    });

    ::syn::parse_quote! { { #(#field_exprs);* } }
}
