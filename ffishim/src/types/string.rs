use ::syn::*;

/// The std lib's `String` type behavior.
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, _: &Type) -> bool { panic!("string is not implemented"); }
    fn fold(&self, _: Type) -> Type { panic!("string fold not implemented"); }
    fn try_into(&self, _: Expr) -> Expr { panic!("string try_into not implemented"); }
    fn from(&self, _: Expr) -> Expr { panic!("string from not implemented"); }
}
