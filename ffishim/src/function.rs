use crate::types::Behavior;
use crate::helpers::*;

impl ::quote::ToTokens for crate::Function {
    fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
        let ffi_name = &self.ffi_name;
        let ffi_args = &self.ffi_args;
        let ffi_output = &self.ffi_output;
        let call_expr = &self.call_expr;

        tokens.extend(::quote::quote! {
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
        ::syn::FnArg::Typed(pat) => ::syn::FnArg::Typed(::syn::PatType{
            ty: Box::new(crate::types::switch(&pat.ty).fold(*pat.ty.clone())),
            ..pat.clone()
        }),
    }
}

fn output_to_ffi_output(output: &::syn::ReturnType) -> ::syn::ReturnType {
    if let ::syn::ReturnType::Type(_, sty) = output {
        if crate::types::Result.is(&sty) {
            let subtype = sty.clone().into_subtype();
            let subtype = crate::types::switch(&subtype).fold(subtype);
            ::syn::parse_quote! { -> *mut ::ffishim::library::Outcome<#subtype> }
        } else {
            let sty = crate::types::switch(&sty).fold(*sty.clone());
            ::syn::parse_quote! { -> *mut ::ffishim::library::Outcome<#sty> }
        }
    } else {
        ::syn::parse_quote! { -> *mut ::ffishim::library::Outcome<()> }
    }
}

fn call_expr_from_item_fn(ifn: &::syn::ItemFn) -> ::syn::Expr {
    let orig_name = &ifn.sig.ident;
    let convert_exprs = ifn.sig.inputs.iter().map(|arg| {
        if let ::syn::FnArg::Typed(pat) = arg {
            let arg_name = pat.unwrap_ident_as_expr();
            crate::types::switch(&pat.ty).try_into(&pat.ty, arg_name)
        } else {
            panic!("no receiver (self) supported in function signatures");
        }
    });

    let call_expr: ::syn::Expr = ::syn::parse_quote! {
        #orig_name(
            #(match #convert_exprs {
                Ok(tmp) => tmp,
                Err(err) => return ::ffishim::library::Outcome::error(err).into_raw(),
            }),*
        )
    };

    if let ::syn::ReturnType::Type(_, sty) = &ifn.sig.output {
        if crate::types::Result.is(&sty) {
            let subtype = sty.clone().into_subtype();
            let receiver: ::syn::Expr = ::syn::parse_quote! { tmp };
            let subexpr = crate::types::switch(&subtype).from(&subtype, receiver.clone());
            ::syn::parse_quote! {
                ::ffishim::library::Outcome::from(#call_expr.map(|#receiver| #subexpr)).into_raw()
            }
        } else {
            let call_expr = crate::types::switch(&sty).from(&sty, call_expr);
            ::syn::parse_quote! { ::ffishim::library::Outcome::success(#call_expr).into_raw() }
        }
    } else {
        ::syn::parse_quote! { { #call_expr; ::ffishim::library::Outcome::success(()).into_raw() } }
    }
}
