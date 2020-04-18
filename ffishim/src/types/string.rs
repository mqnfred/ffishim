use crate::helpers::*;
use ::syn::*;

/// The std lib's `String` type behavior.
///
/// We currently use a `CString` from to ingest all `String` values. This might be unsafe if the string
/// was instantiated by the caller without using rust's instanciation mechanism. In that case, it
/// would be safe to use a `CStr`. See https://doc.rust-lang.org/std/ffi/struct.CString.html
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            is_same_id(&tp.path, "String")
        } else {
            false
        }
    }

    fn fold(&self, _: Type) -> Type {
        parse_quote! { *mut i8 }
    }

    fn try_into(&self, _: &Type, expr: Expr) -> Expr {
        parse_quote! {
            {
                let tmp: Result<String, ::ffishim::library::Error> = unsafe {
                    ::std::ffi::CString::from_raw(#expr)
                }.into_string().map_err(|e| {
                    ::ffishim::library::Error::msg(e.to_string())
                });
                tmp
            }
        }
    }

    fn from(&self, _: &Type, expr: Expr) -> Expr {
        parse_quote! {
            ::std::ffi::CString::new(
                #expr
            ).expect("bad rust string sent through ffi shim").into_raw()
        }
    }
}
