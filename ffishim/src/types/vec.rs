use crate::helpers::*;
use ::syn::*;

/// The std lib's `Vec` type behavior, backed by ffishim library's `FFIVec`.
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            is_same_id(&tp.path, "Vec")
        } else {
            false
        }
    }

    fn fold(&self, sty: Type) -> Type {
        let subtype = sty.into_subtype();
        let ffi_subtype = crate::types::switch(&subtype).fold(subtype);
        parse_quote! { *mut ::ffishim::library::FFIVec<#ffi_subtype> }
    }

    fn try_into(&self, sty: &Type, expr: Expr) -> Expr {
        let orig_subtype = sty.clone().into_subtype();
        let ffi_subtype = crate::types::switch(&orig_subtype).fold(orig_subtype.clone());

        let receiver: Expr = parse_quote! { tmp };
        let subexpr = crate::types::switch(&orig_subtype).try_into(&orig_subtype, receiver.clone());

        parse_quote! {{
            let tmp = #expr;
            if !tmp.is_null() {
                let tmp = *unsafe { Box::from_raw(tmp) };
                ::ffishim::library::FFIVec::<#ffi_subtype>::into_vec(tmp).into_iter().map(|#receiver| {
                    #subexpr
                }).collect::<Result<Vec<_>, ::ffishim::library::Error>>()
            } else {
                Err(::ffishim::library::Error::msg("nil array received"))
            }
        }}
    }

    fn from(&self, sty: &Type, expr: Expr) -> Expr {
        let orig_subtype = sty.clone().into_subtype();
        let ffi_subtype = crate::types::switch(&orig_subtype).fold(orig_subtype.clone());

        let receiver: Expr = parse_quote! { tmp };
        let subexpr = crate::types::switch(&orig_subtype).from(&orig_subtype, receiver.clone());

        parse_quote! {
            Box::into_raw(Box::new(::ffishim::library::FFIVec::<#ffi_subtype>::from(
                #expr.into_iter().map(|#receiver| #subexpr).collect()
            )))
        }
    }

    fn free(&self, sty: &Type, expr: Expr) -> Option<Expr> {
        let orig_subtype = sty.clone().into_subtype();
        let ffi_subtype = crate::types::switch(&orig_subtype).fold(orig_subtype.clone());

        let receiver: Expr = parse_quote! { tmp };
        let subexpr = crate::types::switch(&orig_subtype).free(&orig_subtype, receiver.clone());

        Some(parse_quote!{{
            let tmp = #expr;
            if !tmp.is_null() {
                let tmp = *unsafe { Box::from_raw(tmp) };
                ::ffishim::library::FFIVec::<#ffi_subtype>::into_vec(tmp).into_iter().map(|#receiver| {
                    #subexpr
                }).last();
            }
        }})
    }
}
