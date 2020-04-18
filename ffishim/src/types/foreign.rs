use ::syn::*;

/// Any unknown type's behavior, assumed to implement an ffi shim.
///
/// If a type is unknown (meaning it did not match any pre-defined `Behavior`,) we consider it by
/// default to be a `Custom` type: a type defined by the user that itself implements the ffi shim.
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, _: &Type) -> bool {
        true
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
