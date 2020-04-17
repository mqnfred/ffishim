impl ::quote::ToTokens for crate::Function {
    fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
        
    }
}

impl crate::Function {
    pub fn from_item_fn(ifn: &::syn::ItemFn) -> ::anyhow::Result<Self> {
        Ok(Self {
            ident: ifn.sig.ident.clone(),
            args: ifn.sig.inputs.iter().cloned().collect(),
            ret: ifn.sig.output.clone(),
        })
    }
}
