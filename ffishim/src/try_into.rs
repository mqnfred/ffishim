use crate::helpers::*;

impl ::quote::ToTokens for crate::TryInto {
    fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
        let orig_name = &self.orig_name;
        let ffi_name = &self.ffi_name;
        let receiver = &self.receiver;
        let init_expr = &self.init_expr;

        tokens.extend(::quote::quote! {
            use ::std::convert::TryInto as _;
            impl ::std::convert::TryInto<#orig_name> for #ffi_name {
                type Error = ::ffishim::library::Error;
                fn try_into(#receiver: #ffi_name) -> Result<#orig_name, Self::Error> {
                    #init_expr
                }
            }
        });
    }
}

impl<'a> From<&'a crate::Data> for crate::TryInto {
    fn from(data: &'a crate::Data) -> Self {
        let orig_name = data.ident.clone();
        let ffi_name = data.ident.clone().prefix("FFI");
        let receiver: ::syn::Expr = ::syn::parse_quote! { self };

        let init_expr = match &data.data {
            ::darling::ast::Data::Enum(variants) => enum_init_expr(
                &::syn::parse_quote! { #orig_name },
                &::syn::parse_quote! { #ffi_name },
                &receiver,
                variants,
            ),
            ::darling::ast::Data::Struct(fields) => struct_init_expr(
                &::syn::parse_quote! { #orig_name },
                &data.constructor,
                Some(&receiver),
                fields,
            ),
        };

        Self{orig_name, ffi_name, receiver, init_expr}
    }
}

fn enum_init_expr(
    orig_name: &::syn::Path,
    ffi_name: &::syn::Path,
    receiver: &::syn::Expr,
    variants: &Vec<crate::Variant>,
) -> ::syn::Expr {
    let arms: Vec<::syn::Arm> = variants.iter().map(|v| {
        let variant_name = &v.ident;
        let orig_variant_fullpath: ::syn::Path = ::syn::parse_quote! { #orig_name::#variant_name };
        let ffi_variant_fullpath: ::syn::Path = ::syn::parse_quote! { #ffi_name::#variant_name };

        let init = struct_init_expr(&orig_variant_fullpath, &v.constructor, None, &v.fields);
        let destructuring: Vec<::syn::Ident> = v.fields.iter().enumerate().map(|(idx, field)| {
            field.ident.clone().unwrap_or(idx_to_name(idx as u32))
        }).collect();

        match v.fields.style {
            ::darling::ast::Style::Tuple => ::syn::parse_quote! {
                #ffi_variant_fullpath(#(#destructuring),*) => #init
            },
            ::darling::ast::Style::Struct => ::syn::parse_quote! {
                #ffi_variant_fullpath{#(#destructuring),*} => #init
            },
            ::darling::ast::Style::Unit => ::syn::parse_quote! {
                #ffi_variant_fullpath => Ok(#orig_variant_fullpath),
            },
        }
    }).collect();

    ::syn::parse_quote! {
        match #receiver {
            #(#arms),*
        }
    }
}

fn struct_init_expr(
    orig_name: &::syn::Path,
    constructor: &Option<::syn::Path>,
    receiver: Option<&::syn::Expr>,
    fields: &::darling::ast::Fields<crate::Field>,
) -> ::syn::Expr {
    let exprs = fields.iter().enumerate().map(|(idx, field)| {
        field.try_into(idx, receiver)
    });

    if let Some(constructor) = constructor {
        ::syn::parse_quote! { #constructor(#(#exprs?),*) }
    } else if fields.style == ::darling::ast::Style::Tuple {
        ::syn::parse_quote! { Ok(#orig_name(#(#exprs?),*)) }
    } else {
        let field_inits: Vec<::syn::FieldValue> = fields.iter().zip(exprs).map(|(field, expr)| {
            let field_name = &field.ident.as_ref().unwrap();
            ::syn::parse_quote! { #field_name: #expr? }
        }).collect();
        ::syn::parse_quote! { Ok(#orig_name{#(#field_inits),*}) }
    }
}
