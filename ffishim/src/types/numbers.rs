use ::syn::*;
use crate::helpers::*;

pub struct Behavior;

lazy_static! {
    static ref NUMBER_TYPES: ::std::collections::HashMap<&'static str, &'static str> = {
        let mut m = ::std::collections::HashMap::new();
        m.insert("char", "c_char");
        m.insert("f32", "c_float");
        m.insert("f64", "c_double");
        m.insert("u8", "c_char");
        m.insert("u16", "c_ushort");
        m.insert("u32", "c_uint");
        m.insert("u64", "c_ulong");
        m.insert("usize", "size_t");
        m.insert("i8", "c_schar");
        m.insert("i16", "c_short");
        m.insert("i32", "c_int");
        m.insert("i64", "c_long");
        m.insert("isize", "ssize_t");
        m
    };
}

impl crate::TypeBehavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            NUMBER_TYPES.keys().any(|name| is_same_id(&tp.path, name))
        } else {
            false
        }
    }

    fn fold(&self, sty: Type) -> Type {
        panic!("wow fold sty numbers");
    }

    fn try_into(&self, _: Expr) -> Expr {
        panic!("wow try_into sty umbers");
    }

    fn from(&self, expr: Expr) -> Expr {
        expr
    }
}
