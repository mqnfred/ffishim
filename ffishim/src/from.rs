use crate::helpers::*;

impl ::quote::ToTokens for crate::From {
    fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
        let orig_name = &self.orig_name;
        let ffi_name = &self.ffi_name;
        let receiver = &self.receiver;
        let init_expr = &self.init_expr;

        tokens.extend(::quote::quote! {
            impl From<#orig_name> for #ffi_name {
                fn from(#receiver: #orig_name) -> #ffi_name {
                    #init_expr
                }
            }
        });
    }
}

impl<'a> From<&'a crate::Data> for crate::From {
    fn from(data: &'a crate::Data) -> Self {
        let orig_name = data.ident.clone();
        let ffi_name = data.ident.clone().prefix("FFI");
        let receiver: ::syn::Expr = ::syn::parse_quote! { tmp };

        let init_expr = match &data.data {
            ::darling::ast::Data::Enum(variants) => enum_init_expr(
                &::syn::parse_quote! { #orig_name },
                &::syn::parse_quote! { #ffi_name },
                &receiver,
                variants,
            ),
            ::darling::ast::Data::Struct(fields) => struct_init_expr(
                &::syn::parse_quote! { #ffi_name },
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

        let init = struct_init_expr(&ffi_variant_fullpath, None, &v.fields);
        let destructuring: Vec<::syn::Ident> = v.fields.iter().enumerate().map(|(idx, field)| {
            field.ident.clone().unwrap_or(idx_to_name(idx as u32))
        }).collect();

        match v.fields.style {
            ::darling::ast::Style::Tuple => ::syn::parse_quote! {
                #orig_variant_fullpath(#(#destructuring),*) => #init
            },
            ::darling::ast::Style::Struct => ::syn::parse_quote! {
                #orig_variant_fullpath{#(#destructuring),*} => #init
            },
            ::darling::ast::Style::Unit => ::syn::parse_quote! {
                #orig_variant_fullpath => #ffi_variant_fullpath,
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
    ffi_name: &::syn::Path,
    receiver: Option<&::syn::Expr>,
    fields: &::darling::ast::Fields<crate::Field>,
) -> ::syn::Expr {
    let exprs = fields.iter().enumerate().map(|(idx, field)| {
        field.from(idx, receiver)
    });

    if fields.style == ::darling::ast::Style::Tuple {
        ::syn::parse_quote! { #ffi_name(#(#exprs),*) }
    } else {
        let field_inits: Vec<::syn::FieldValue> = fields.iter().zip(exprs).map(|(field, expr)| {
            let field_name = &field.ident.as_ref().unwrap();
            ::syn::parse_quote! { #field_name: #expr }
        }).collect();
        ::syn::parse_quote! { #ffi_name{#(#field_inits),*} }
    }
}
