use crate::helpers::*;

impl ::quote::ToTokens for crate::Data {
    fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
        let ffi_name = self.ident.clone().prefix("FFI");

        if self.opaque {
            self.opaque_to_tokens(ffi_name, tokens)
        } else {
            match &self.data {
                ::darling::ast::Data::Struct(fds) => self.struct_to_tokens(ffi_name, fds, tokens),
                ::darling::ast::Data::Enum(vars) => self.enum_to_tokens(ffi_name, vars, tokens),
            }
        }
    }
}

impl crate::Data {
    pub(crate) fn validate(self) -> Self {
        if let ::darling::ast::Data::Enum(_) = &self.data {
            if self.constructor.is_some() {
                panic!("in enums, please specifies constructors per-variant");
            }
        }
        self
    }

    fn opaque_to_tokens(&self, ffi_name: ::syn::Ident, tokens: &mut ::proc_macro2::TokenStream) {
        let orig_name = &self.ident;
        tokens.extend(::quote::quote! {
            pub struct #ffi_name(#orig_name);
        });
    }

    fn enum_to_tokens(
        &self,
        ffi_name: ::syn::Ident,
        variants: &Vec<crate::Variant>,
        tokens: &mut ::proc_macro2::TokenStream,
    ) {
        let variants: Vec<_> = variants.iter().map(|v| v.fold()).collect();
        tokens.extend(::quote::quote! {
            #[repr(C, u16)]
            pub enum #ffi_name {
                #(#variants),*
            }
        });
    }

    fn struct_to_tokens(
        &self,
        ffi_name: ::syn::Ident,
        fields: &::darling::ast::Fields<crate::Field>,
        tokens: &mut ::proc_macro2::TokenStream,
    ) {
        let fields: Vec<::syn::Field> = fields.iter().map(|f| f.fold()).collect();
        tokens.extend(::quote::quote! {
            #[repr(C)]
            pub struct #ffi_name {
                #(#fields),*
            }
        });
    }
}
