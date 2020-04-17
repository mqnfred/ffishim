use ::once_cell::sync::Lazy;
use ::std::collections::HashMap;
use ::syn::*;

/// Builtin scalar behaviors: `char`, `f32`, `u32`, ...
///
/// The behavior for different scalars is shared into this object. Here is the list of scalars and
/// their libc equivalents:
///
///  - `char` -> `c_char`
///  - `f32` -> `c_float`
///  - `f64` -> `c_double`
///  - `u8` -> `c_char`
///  - `u16` -> `c_ushort`
///  - `u32` -> `c_uint`
///  - `u64` -> `c_ulong`
///  - `usize` -> `size_t`
///  - `i8` -> `c_schar`
///  - `i16` -> `c_short`
///  - `i32` -> `c_int`
///  - `i64` -> `c_long`
///  - `isize` -> `ssize_t`
pub struct Behavior;

static NUMBER_TYPES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("char", "::ffishim::library::libc::c_char");
    m.insert("f32", "::ffishim::library::libc::c_float");
    m.insert("f64", "::ffishim::library::libc::c_double");
    m.insert("u8", "::ffishim::library::libc::c_char");
    m.insert("u16", "::ffishim::library::libc::c_ushort");
    m.insert("u32", "::ffishim::library::libc::c_uint");
    m.insert("u64", "::ffishim::library::libc::c_ulong");
    m.insert("usize", "::ffishim::library::libc::size_t");
    m.insert("i8", "::ffishim::library::libc::c_schar");
    m.insert("i16", "::ffishim::library::libc::c_short");
    m.insert("i32", "::ffishim::library::libc::c_int");
    m.insert("i64", "::ffishim::library::libc::c_long");
    m.insert("isize", "::ffishim::library::libc::ssize_t");
    m
});

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            NUMBER_TYPES.keys().any(|t| {
                ::syn::parse_str::<::syn::Path>(t).unwrap() == tp.path
            })
        } else {
            false
        }
    }

    fn fold(&self, sty: Type) -> Type {
        if let Type::Path(tp) = sty {
            ::syn::parse_str::<::syn::Type>(
                NUMBER_TYPES.get(tp.path.get_ident().unwrap().to_string().as_str()).unwrap()
            ).unwrap()
        } else {
            panic!("expected type path for numbers");
        }
    }

    fn try_into(&self, expr: Expr) -> Expr {
        ::syn::parse_quote! { Ok(#expr) }
    }

    fn from(&self, expr: Expr) -> Expr {
        expr
    }
}
