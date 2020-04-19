use crate::helpers::*;
use ::syn::*;

/// The std lib's `Vec` type behavior, backed by ffishim library's `Array`.
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            is_same_id(&tp.path, "Vec")
        } else {
            false
        }
    }

    fn fold(&self, _: Type) -> Type {
        panic!("vec fold not implemented");
    }

    fn try_into(&self, _: &Type, _: Expr) -> Expr {
        panic!("vec try_into not implemented");
    }

    fn from(&self, _: &Type, _: Expr) -> Expr {
        panic!("vec from not implemented");
    }
}
