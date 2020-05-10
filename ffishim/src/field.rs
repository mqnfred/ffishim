use crate::helpers::*;

impl crate::Field {
    pub(crate) fn fold(&self) -> ::syn::Field {
        ::syn::Field {
            attrs: vec![],
            vis: self.vis.clone(),
            ident: self.ident.clone(),
            colon_token: None,
            ty: if self.opaque {
                let ty = &self.ty;
                ::syn::parse_quote! { *mut #ty }
            } else {
                crate::types::switch(&self.ty).fold(self.ty.clone())
            },
        }
    }

    pub(crate) fn from(&self, idx: usize, receiver: Option<&::syn::Expr>) -> ::syn::Expr {
        let receiver = self.build_receiver(idx, receiver);
        if self.opaque {
            ::syn::parse_quote! { Box::into_raw(Box::new(#receiver)) }
        } else {
            crate::types::switch(&self.ty).from(&self.ty, receiver)
        }
    }

    pub(crate) fn try_into(&self, idx: usize, receiver: Option<&::syn::Expr>) -> ::syn::Expr {
        let receiver = self.build_receiver(idx, receiver);
        if self.opaque {
            ::syn::parse_quote! {{
                let tmp = #receiver;
                if tmp.is_null() {
                    Err(::ffishim::library::Error::msg("empty opaque field provided"))
                } else {
                    Ok(*unsafe { Box::from_raw(#receiver) })
                }
            }}
        } else {
            crate::types::switch(&self.ty).try_into(&self.ty, receiver)
        }
    }

    pub(crate) fn free(&self, idx: usize, receiver: Option<&::syn::Expr>) -> Option<::syn::Expr> {
        let receiver = self.build_receiver(idx, receiver);
        if self.opaque {
            Some(::syn::parse_quote! {{
                let tmp = #receiver;
                if !tmp.is_null() {
                    unsafe { Box::from_raw(tmp) };
                }
            }})
        } else {
            crate::types::switch(&self.ty).free(&self.ty, receiver)
        }
    }

    fn build_receiver(&self, idx: usize, receiver: Option<&::syn::Expr>) -> ::syn::Expr {
        if let Some(receiver) = receiver {
            if let Some(ident) = self.ident.as_ref() {
                ::syn::parse_quote! { #receiver.#ident }
            } else {
                idx_fieldexpr(idx as u32, receiver)
            }
        } else {
            if let Some(ident) = self.ident.as_ref() {
                ::syn::parse_quote! { #ident }
            } else {
                let idx_ident = idx_to_name(idx as u32);
                ::syn::parse_quote! { #idx_ident }
            }
        }
    }
}

fn idx_fieldexpr(idx: u32, receiver: &::syn::Expr) -> ::syn::Expr {
    ::syn::Expr::Field(::syn::ExprField {
        attrs: vec![],
        base: ::syn::parse_quote! { #receiver },
        dot_token: ::syn::Token!(.)(::proc_macro2::Span::call_site()),
        member: ::syn::Member::Unnamed(::syn::Index{
            index: idx,
            span: ::proc_macro2::Span::call_site(),
        }),
    })
}
