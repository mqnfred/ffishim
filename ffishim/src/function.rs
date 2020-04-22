use crate::types::Behavior;
use crate::helpers::*;

impl ::quote::ToTokens for crate::Function {
    fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
        let ffi_name = &self.ffi_name;
        let ffi_args = &self.ffi_args;
        let ffi_output = &self.ffi_output;
        let call_expr = &self.call_expr;

        tokens.extend(::quote::quote! {
            #[allow(unused_imports)]
            use ::std::convert::TryInto as _;

            #[no_mangle]
            pub extern "C" fn #ffi_name(#(#ffi_args),*) #ffi_output {
                #call_expr
            }
        });
    }
}

impl crate::Function {
    pub fn from_item_fn(ifn: &::syn::ItemFn) -> Self {
        Self {
            ffi_name: ifn.sig.ident.clone().prefix("ffi_"),
            ffi_args: ifn.sig.inputs.iter().map(|arg| arg_to_ffi_arg(arg)).collect(),
            ffi_output: output_to_ffi_output(&ifn.sig.output),
            call_expr: call_expr_from_item_fn(ifn),
        }
    }
}

fn arg_to_ffi_arg(arg: &::syn::FnArg) -> ::syn::FnArg {
    match arg {
        ::syn::FnArg::Receiver(_) => panic!("self receiver not handled in functions"),
        ::syn::FnArg::Typed(patty) => ::syn::FnArg::Typed(::syn::PatType{
            pat: Box::new(*patty.pat.clone()),
            ty: Box::new(crate::types::switch(&patty.ty).fold(*patty.ty.clone())),
            ..patty.clone()
        }),
    }
}

fn output_to_ffi_output(output: &::syn::ReturnType) -> ::syn::ReturnType {
    let sty = if let ::syn::ReturnType::Type(_, sty) = output {
        if crate::types::Result.is(&sty) {
            crate::types::Result.fold(*sty.clone())
        } else {
            crate::types::Result.fold(::syn::parse_quote! { Result<#sty, ::ffishim::library::Error> })
        }
    } else {
        crate::types::Result.fold(::syn::parse_quote! { Result<(), ::ffishim::library::Error> })
    };

    ::syn::parse_quote! { -> #sty }
}

fn call_expr_from_item_fn(ifn: &::syn::ItemFn) -> ::syn::Expr {
    let orig_name = &ifn.sig.ident;
    let convert_exprs = ifn.sig.inputs.iter().map(|arg| {
        if let ::syn::FnArg::Typed(pat) = arg {
            let arg_name = pat.unwrap_ident_as_expr();
            let expr = crate::types::switch(&pat.ty).try_into(&pat.ty, arg_name);
            crate::types::Result.try_or_return(expr)
        } else {
            panic!("no receiver (self) supported in function signatures");
        }
    });

    let call_expr: ::syn::Expr = ::syn::parse_quote! { #orig_name(#(#convert_exprs),*) };

    if let ::syn::ReturnType::Type(_, sty) = &ifn.sig.output {
        if crate::types::Result.is(&sty) {
            crate::types::Result.from(sty, call_expr)
        } else {
            crate::types::Result.wrap_success(sty, call_expr)
        }
    } else {
        crate::types::Result.wrap_success(&::syn::parse_quote! { () }, call_expr)
    }
}
