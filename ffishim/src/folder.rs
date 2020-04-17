use ::syn::*;
use crate::helpers::*;
use crate::TypeBehavior;

impl fold::Fold for crate::Folder {
    fn fold_derive_input(&mut self, di: DeriveInput) -> DeriveInput {
        let mut attrs = di.attrs.into_iter().map(|attr| {
            self.fold_attribute(attr)
        }).collect::<Vec<Attribute>>();
        attrs.push(parse_quote! { #[repr(C)] });

        DeriveInput {
            attrs,
            vis: self.fold_visibility(di.vis),
            ident: self.fold_ident(di.ident.prefix("FFI")),
            generics: self.fold_generics(di.generics),
            data: self.fold_data(di.data),
        }
    }

    fn fold_item_struct(&mut self, is: ItemStruct) -> ItemStruct {
        let mut attrs = is.attrs.into_iter().map(|attr| {
            self.fold_attribute(attr)
        }).collect::<Vec<Attribute>>();
        attrs.push(parse_quote! { #[repr(C)] });

        ItemStruct {
            attrs,
            vis: self.fold_visibility(is.vis),
            struct_token: is.struct_token,
            ident: self.fold_ident(is.ident.prefix("FFI")),
            generics: self.fold_generics(is.generics),
            fields: self.fold_fields(is.fields),
            semi_token: is.semi_token,
        }
    }

    fn fold_item_enum(&mut self, ie: ItemEnum) -> ItemEnum {
        let mut attrs = ie.attrs.into_iter().map(|attr| {
            self.fold_attribute(attr)
        }).collect::<Vec<Attribute>>();
        attrs.push(parse_quote! { #[repr(C)] });

        ItemEnum {
            attrs,
            vis: self.fold_visibility(ie.vis),
            enum_token: ie.enum_token,
            ident: self.fold_ident(ie.ident.prefix("FFI")),
            generics: self.fold_generics(ie.generics),
            brace_token: ie.brace_token,
            variants: ie.variants.into_iter().map(|v| self.fold_variant(v)).collect(),
        }
    }

    fn fold_item_union(&mut self, _: ItemUnion) -> ItemUnion {
        panic!("unions not supported");
    }

    fn fold_data_union(&mut self, _: DataUnion) -> DataUnion {
        panic!("unions not supported");
    }

    fn fold_type(&mut self, sty: Type) -> Type {
        crate::switch(&sty).fold(sty)
    }

    fn fold_item_fn(&mut self, ifn: ItemFn) -> ItemFn {
        let mut attrs = ifn.attrs.into_iter().map(|attr| {
            self.fold_attribute(attr)
        }).collect::<Vec<Attribute>>();
        attrs.push(parse_quote! { #[no_mangle] });

        ItemFn {
            attrs,
            vis: self.fold_visibility(ifn.vis),
            sig: self.fold_signature(ifn.sig),
            block: Box::new(self.fold_block(*ifn.block)),
        }
    }

    fn fold_signature(&mut self, sig: Signature) -> Signature {
        let inputs = sig.inputs.into_iter().map(|i| self.fold_fn_arg(i)).collect();

        let sig_ident = &sig.ident;
        let convert_exprs = self.convert_exprs.drain(..); // filled by fold_fn_arg call
        self.call_expr = Some(parse_quote! {
            #sig_ident(
                #(
                    match #convert_exprs {
                        Ok(tmp) => tmp,
                        Err(err) => return ::cardinal::libcshim::Outcome::from_err(err),
                    }
                ),*
            )
        });

        Signature {
            constness: sig.constness,
            asyncness: sig.asyncness,
            unsafety: sig.unsafety,
            abi: Some(parse_quote! { extern "C" }),
            fn_token: sig.fn_token,
            ident: self.fold_ident(sig.ident.prefix("ffi_")),
            generics: self.fold_generics(sig.generics),
            paren_token: sig.paren_token,
            inputs,
            variadic: sig.variadic.map(|v| self.fold_variadic(v)),
            output: self.fold_return_type(sig.output),
        }
    }

    fn fold_fn_arg(&mut self, arg: FnArg) -> FnArg {
        if let FnArg::Typed(pat) = arg {
            let tyb = crate::switch(&pat.ty);

            // add conversion expression to list
            let arg_name = pat.unwrap_ident_as_expr();
            self.convert_exprs.push(tyb.try_into(arg_name));

            FnArg::Typed(PatType {
                attrs: pat.attrs.into_iter().map(|a| self.fold_attribute(a)).collect(),
                pat: Box::new(self.fold_pat(*pat.pat)),
                colon_token: pat.colon_token,
                ty: Box::new(tyb.fold(*pat.ty)),
            })
        } else {
            panic!("no receiver (self) supported in function signatures");
        }
    }

    fn fold_return_type(&mut self, rt: ReturnType) -> ReturnType {
        if let ReturnType::Type(_, sty) = rt {
            let sty = if crate::types::result::Behavior.is(&sty) {
                let subtype = sty.clone().into_subtype();

                // convert return type of call_expr inside result and mutate result into outcome
                let se = crate::switch(&subtype).from(parse_quote! { tmp });
                self.call_expr = self.call_expr.take().map(|ce| {
                    parse_quote! { ::ffficshim::Outcome::from(#ce.map(|tmp| #se)).into_raw() }
                });

                subtype
            } else {
                // convert return type of call_expr and wrap inside success
                self.call_expr = self.call_expr.take().map(|ce| {
                    let ce = crate::switch(&sty).from(ce);
                    parse_quote! { ::ffficshim::Outcome::success(#ce).into_raw() }
                });

                *sty
            };

            // fold return type into an outcome of itself (or its child if result)
            let sty = crate::switch(&sty).fold(sty);
            parse_quote!{ -> *mut ::ffishim::library::Outcome<#sty> }
        } else {
            // if call_expr returns nothing, discard it and return success every time
            self.call_expr = self.call_expr.take().map(|ce| {
                parse_quote! { { #ce; ::ffishim::library::Outcome::success(()).into_raw() } }
            });

            parse_quote!{ -> *mut ::ffishim::library::Outcome<()> }
        }
    }

    fn fold_block(&mut self, bl: Block) -> Block {
        let call_expr = self.call_expr.take().expect("filled earlier by signature/ret folding");
        Block {
            brace_token: bl.brace_token,
            stmts: vec![parse_quote!{ #call_expr }],
        }
    }
}
