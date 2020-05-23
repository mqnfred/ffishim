use ::syn::*;
use crate::helpers::*;

/// Behavior for references to types (& and &mut).
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Reference(_) = sty {
            true
        } else {
            false
        }
    }

    fn fold(&self, sty: Type) -> Type {
        let subtype = inner_type(&sty);
        let ffi_subtype = crate::types::switch(subtype).fold(subtype.clone());
        parse_quote! { #ffi_subtype }
    }

    fn try_into(&self, sty: &Type, expr: Expr) -> Expr {
        if let Type::Path(mut tp) = inner_type(&sty).clone() {
            let seg = tp.path.segments.last_mut().expect(">0 segments");
            seg.ident = seg.ident.clone().prefix("FFI");
            parse_quote! {{
                let tmp: &mut #tp = unsafe { &mut *#expr };
                Ok(&mut tmp.0)
            }}
        } else {
            panic!("can only reference to opaque types");
        }
    }

    fn from(&self, _sty: &Type, _expr: Expr) -> Expr {
        panic!("from not supported for reference types");
    }

    fn free(&self, _sty: &Type, _expr: Expr) -> Option<Expr> {
        None
    }
}

fn inner_type(sty: &Type) -> &Type {
    if let Type::Reference(r) = sty {
        &*r.elem
    } else {
        panic!("unexpected non-reference type")
    }
}
