impl crate::Variant {
    pub(crate) fn fold(&self) -> ::syn::Variant {
        let variant_name = &self.ident;

        if self.fields.style == ::darling::ast::Style::Tuple {
            let fields: Vec<_> = self.fields.iter().map(|f| f.fold()).collect();
            ::syn::parse_quote! { #variant_name(#(#fields),*) }
        } else if self.fields.style == ::darling::ast::Style::Struct {
            let fields: Vec<_> = self.fields.iter().map(|f| f.fold()).collect();
            ::syn::parse_quote! { #variant_name{#(#fields),*} }
        } else {
            ::syn::parse_quote! { #variant_name }
        }
    }
}
