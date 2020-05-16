use crate::helpers::*;
use ::syn::*;

/// The std lib's `char` type behavior.
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            is_same_id(&tp.path, "char")
        } else {
            false
        }
    }

    fn fold(&self, _sty: Type) -> Type {
        parse_quote! { ::ffishim::library::libc::c_uint }
    }

    fn try_into(&self, _sty: &Type, expr: Expr) -> Expr {
        parse_quote! { Ok(#expr as ::ffishim::library::libc::c_uint) }
    }

    fn from(&self, _sty: &Type, expr: Expr) -> Expr {
        parse_quote! { (#expr as char) }
    }

    fn free(&self, _sty: &Type, _expr: Expr) -> Option<Expr> {
        None
    }
}
