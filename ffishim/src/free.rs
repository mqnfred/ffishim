use crate::helpers::*;

impl ::quote::ToTokens for crate::Free {
    fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
        todo!()
    }
}

impl<'a> From<&'a crate::Data> for crate::Free {
    fn from(data: &'a crate::Data) -> Self {
        todo!()
    }
}
