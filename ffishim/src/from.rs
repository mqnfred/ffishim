use ::syn::*;
use crate::helpers::*;

impl<'ast> visit::Visit<'ast> for crate::From {
    fn visit_derive_input(&mut self, di: &'ast DeriveInput) {
        self.name = Some(di.ident.clone());

        for attr in &di.attrs {
            self.visit_attribute(&attr);
        }
        self.visit_visibility(&di.vis);
        self.visit_generics(&di.generics);
        self.visit_data(&di.data);
    }

    fn visit_item_struct(&mut self, is: &'ast ItemStruct) {
        self.name = Some(is.ident.clone());
    }

    fn visit_item_enum(&mut self, ie: &'ast ItemEnum) {
        self.name = Some(ie.ident.clone());
    }

    fn visit_item_union(&mut self, _: &'ast ItemUnion) {
        panic!("unions not supported");
    }

    fn visit_data_union(&mut self, _: &'ast DataUnion) {
        panic!("unions not supported");
    }
}

impl crate::From {
    fn generate(mut self) -> ItemImpl {
        let receiver = self.receiver.take().expect("need to visit struct before calling generate");
        let orig_name = self.name.take().expect("need to visit struct before calling generate");
        let ffi_name = orig_name.clone().prefix("FFI");

        parse_quote! {
            impl From<#orig_name> for #ffi_name {
                fn from(#receiver: #orig_name) -> Self {
                    panic!("hehe");
                }
            }
        }
    }
}
