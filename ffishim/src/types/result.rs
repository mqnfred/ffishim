use ::syn::*;

pub struct Behavior;

impl crate::TypeBehavior for Behavior {
    fn is(&self, _: &Type) -> bool { panic!("not implemented"); }
    fn fold(&self, _: Type) -> Type { panic!("not implemented"); }
    fn try_into(&self, _: Expr) -> Expr { panic!("not implemented"); }
    fn from(&self, _: Expr) -> Expr { panic!("not implemented"); }
}
