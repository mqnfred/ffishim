use crate::helpers::*;

impl crate::Field {
    pub(crate) fn fold(&self) -> ::syn::Field {
        ::syn::Field {
            attrs: vec![],
            vis: self.vis.clone(),
            ident: self.ident.clone(),
            colon_token: None,
            ty: crate::types::switch(&self.ty).fold(self.ty.clone()),
        }
    }

    pub(crate) fn from(&self, idx: usize, receiver: Option<&::syn::Expr>) -> ::syn::Expr {
        let expr: ::syn::Expr = if let Some(receiver) = receiver {
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
        };

        crate::types::switch(&self.ty).from(expr)
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
