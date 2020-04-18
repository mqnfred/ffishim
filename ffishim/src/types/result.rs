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

    fn fold(&self, _: Type) -> Type {
        panic!("result fold not implemented");
    }

    fn try_into(&self, _: &Type, _: Expr) -> Expr {
        panic!("result try_into not implemented");
    }

    fn from(&self, _: &Type, _: Expr) -> Expr {
        panic!("result from not implemented");
    }
}
