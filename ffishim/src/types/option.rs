use ::syn::*;

/// The std lib's `Option` type behavior.
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, _: &Type) -> bool {
        false
    }

    fn fold(&self, _: Type) -> Type { panic!("result fold not implemented"); }
    fn try_into(&self, _: Expr) -> Expr { panic!("result try_into not implemented"); }
    fn from(&self, _: Expr) -> Expr { panic!("result from not implemented"); }
}
