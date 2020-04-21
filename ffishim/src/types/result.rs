use crate::helpers::*;
use ::syn::*;

/// The std lib's `Result` type behavior.
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            is_same_id(&tp.path, "Result")
        } else {
            false
        }
    }

    fn fold(&self, sty: Type) -> Type {
        let subtype = sty.into_subtype();
        let subtype = crate::types::switch(&subtype).fold(subtype);
        parse_quote! { *mut ::ffishim::library::Result<#subtype> }
    }

    fn try_into(&self, _: &Type, _: Expr) -> Expr {
        panic!("cannot pass results as arguments");
    }

    fn from(&self, sty: &Type, expr: Expr) -> Expr {
        let subtype = sty.clone().into_subtype();
        let receiver: ::syn::Expr = ::syn::parse_quote! { tmp };
        let subexpr = crate::types::switch(&subtype).from(&subtype, receiver.clone());
        ::syn::parse_quote! {
            ::ffishim::library::Result::from(#expr.map(|#receiver| #subexpr)).into_raw()
        }
    }

    fn free(&self, _: &Type, expr: Expr) -> Option<Expr> {
        Some(parse_quote! { ::ffishim::library::free_result(#expr) })
    }
}

impl Behavior {
    /// Returns an expression that tries to unpack a `Result`.
    ///
    /// Upon failure, returns *directly* an error-full `Result`. Can be used to unpack arguments in
    /// a ffi wrapper that returns a `Result`.
    pub fn try_or_return(&self, expr: Expr) -> Expr {
        ::syn::parse_quote! {
            match #expr {
                Ok(tmp) => tmp,
                Err(err) => return ::ffishim::library::Result::error(err).into_raw(),
            }
        }
    }

    /// Returns an expression that wraps the given `expr` into a `Result` that is always
    /// successful.
    pub fn wrap_success(&self, sty: &Type, expr: Expr) -> Expr {
        let expr = crate::types::switch(&sty).from(&sty, expr);
        ::syn::parse_quote! { ::ffishim::library::Result::success(#expr).into_raw() }
    }
}
