use crate::helpers::*;
use ::syn::*;

/// The std lib's `bool` type behavior.
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            is_same_id(&tp.path, "bool")
        } else {
            false
        }
    }

    fn fold(&self, _sty: Type) -> Type {
        parse_quote! { ::ffishim::library::libc::c_char }
    }

    fn try_into(&self, _sty: &Type, expr: Expr) -> Expr {
        parse_quote! { Ok(if #expr == 0 { false } else { true }) }
    }

    fn from(&self, _sty: &Type, expr: Expr) -> Expr {
        parse_quote! { if #expr { 1 } else { 0 } }
    }

    fn free(&self, _sty: &Type, _expr: Expr) -> Option<Expr> {
        None
    }
}
