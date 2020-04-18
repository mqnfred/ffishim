use ::syn::*;
use crate::helpers::*;

/// The std lib's `Option` type behavior.
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            is_same_id(&tp.path, "Option")
        } else {
            false
        }
    }

    fn fold(&self, sty: Type) -> Type {
        let subtype = sty.into_subtype();
        let ffi_subtype = crate::types::switch(&subtype).fold(subtype);
        parse_quote! { *mut #ffi_subtype }
    }

    fn try_into(&self, sty: &Type, expr: Expr) -> Expr {
        let subtype = sty.clone().into_subtype();
        let receiver: Expr = parse_quote! { tmp };
        let subexpr = crate::types::switch(&subtype).try_into(&subtype, receiver.clone());

        parse_quote! {
            {
                let tmp: Result<#sty, ::ffishim::library::Error>;
                if #expr == ::std::ptr::null_mut() {
                    tmp = Ok(None)
                } else {
                    tmp = Some(unsafe {
                        let #receiver = *Box::from_raw(#expr); #subexpr
                    }).transpose()
                };
                tmp
            }
        }
    }

    fn from(&self, sty: &Type, expr: Expr) -> Expr {
        let subtype = sty.clone().into_subtype();
        let receiver: Expr = parse_quote! { tmp };
        let subexpr = crate::types::switch(&subtype).from(&subtype, receiver.clone());

        parse_quote! {
            if let Some(#receiver) = #expr {
                Box::into_raw(Box::new(#subexpr))
            } else {
                ::std::ptr::null_mut()
            }
        }
    }
}
