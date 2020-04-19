use ::syn::*;
use crate::helpers::*;

/// Any unknown type's behavior, assumed to implement an ffi shim.
///
/// If a type is unknown (meaning it did not match any pre-defined `Behavior`,) we consider it by
/// default to be a `Custom` type: a type defined by the user that itself implements the ffi shim.
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, _: &Type) -> bool {
        true
    }

    fn fold(&self, sty: Type) -> Type {
        if let Type::Path(mut tp) = sty {
            let seg = tp.path.segments.last_mut().expect(">0 segments");
            seg.ident = seg.ident.clone().prefix("FFI");
            Type::Path(tp)
        } else {
            panic!("only foreign types of type path supported");
        }
    }

    fn try_into(&self, sty: &Type, expr: Expr) -> Expr {
        parse_quote! {{ let t: Result<#sty, ::ffishim::library::Error> = #expr.try_into(); t }}
    }

    fn from(&self, _: &Type, expr: Expr) -> Expr {
        parse_quote! { #expr.into() }
    }
}
