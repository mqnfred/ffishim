impl ::quote::ToTokens for crate::TryInto {
    fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
        panic!("try_into to_tokens");
    }
}

impl<'a> From<&'a crate::Data> for crate::TryInto {
    fn from(data: &'a crate::Data) -> Self {
        panic!("from try_into");
    }
}
