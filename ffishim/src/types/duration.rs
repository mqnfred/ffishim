use ::syn::*;
use crate::helpers::*;

/// chrono::Duration type behavior
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            is_same_id(&tp.path, "Duration")
        } else {
            false
        }
    }

    fn fold(&self, _sty: Type) -> Type {
        parse_quote! { i64 }
    }

    fn try_into(&self, _sty: &Type, expr: Expr) -> Expr {
        parse_quote! {{
            let tmp = ::chrono::Duration::milliseconds(#expr);
            let tmp: Result<_, ::ffishim::library::Error> = Ok(tmp);
            tmp
        }}
    }

    fn from(&self, _sty: &Type, expr: Expr) -> Expr {
        parse_quote! { #expr.num_milliseconds() }
    }

    fn free(&self, _sty: &Type, _expr: Expr) -> Option<Expr> {
        None
    }
}
