use syn::*;

/// This macro can be used to add functionality to some nodes in the `syn` AST.
///
/// For some examples, look at some helpers in this module.
macro_rules! extend_syn_node {
    (
        $name:ident =>
            fn $method_name:ident(
                $self:ident
                $(, $arg:ident: $ty:ty)*
            ) $(-> $ret:ty)? $body:block
    ) => {
        ::paste::item_with_macros! {
            #[allow(non_camel_case_types)]
            pub trait [< $name E x t _ $method_name >] {
                fn $method_name($self$(, $arg: $ty)*) $(-> $ret)?;
            }

            impl [< $name E x t _ $method_name >] for $name {
                fn $method_name($self$(, $arg: $ty)*) $(-> $ret)? $body
            }
        }
    };

    (
        $name:ident =>
            fn $method_name:ident(
                &$self:ident
                $(, $arg:ident: $ty:ty)*
            ) $(-> $ret:ty)? $body:block
    ) => {
        ::paste::item_with_macros! {
            #[allow(non_camel_case_types)]
            pub trait [< $name E x t _ $method_name >] {
                fn $method_name(&$self$(, $arg: $ty)*) $(-> $ret)?;
            }

            impl [< $name E x t _ $method_name >] for $name {
                fn $method_name(&$self$(, $arg: $ty)*) $(-> $ret)? $body
            }
        }
    };

    (
        $name:ident =>
            fn $method_name:ident(
                &mut $self:ident
                $(, $arg:ident: $ty:ty)*
            ) $(-> $ret:ty)? $body:block
    ) => {
        ::paste::item_with_macros! {
            #[allow(non_camel_case_types)]
            pub trait [< $name E x t _ $method_name >] {
                fn $method_name(&mut $self$(, $arg: $ty)*) $(-> $ret)?;
            }

            impl [< $name E x t _ $method_name >] for $name {
                fn $method_name(&mut $self$(, $arg: $ty)*) $(-> $ret)? $body
            }
        }
    };
}

extend_syn_node! {
    Type => fn into_subtype(self) -> Type {
        let segments = self.into_typepath_path().segments;
        let segment = segments.into_iter().last().expect("always >0 elements in type path");

        let mut args = match segment.arguments {
            PathArguments::AngleBracketed(arguments) => arguments.args.into_iter(),
            PathArguments::None => panic!("expecting subtype"),
            _ => panic!("only bracketed arguments are supported"),
        };

        match args.next().expect("expecting subtype") {
            GenericArgument::Type(ty) => ty,
            _ => panic!("only the type arguments are supported"),
        }
    }
}

extend_syn_node! {
    Type => fn into_typepath_path(self) -> Path {
        if let Type::Path(tp) = self {
            tp.path
        } else {
            panic!("only normal type of kind typepath supported")
        }
    }
}

extend_syn_node! {
    PatType => fn unwrap_ident_as_expr(&self) -> Expr {
        match *self.pat.clone() {
            Pat::Ident(patid) => { let id = patid.ident.clone(); parse_quote!{ #id } },
            _ => panic!("only supports ident patterns"),
        }
    }
}

extend_syn_node! {
    Ident => fn prefix(self, prefix: &str) -> Ident {
        Ident::new(&format!("{}{}", prefix, self), self.span())
    }
}

pub fn idx_to_name(idx: u32) -> ::syn::Ident {
    new_ident(alpha_idx(idx))
}

pub fn alpha_idx(idx: u32) -> &'static str {
    match idx {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => panic!("does not support more than 20 tuple struct fields"),
    }
}

pub fn new_ident(src: &str) -> Ident {
    Ident::new(src, ::proc_macro2::Span::call_site())
}

pub fn is_same_id(path: &Path, id: &str) -> bool {
    id == &path
        .segments
        .last()
        .expect("always >1 segments")
        .ident
        .to_string()
}

/// `TokenStream` which, when executed, sets the allocator to a known default.
///
/// This is useful to ensure we use the same allocator as the system allocator. The data moving
/// across the FFI boundary sees its ownership move along with it. Some pieces of the host system
/// might need to allocate/free chunks of data it is passing to the rust library.
pub fn shim_allocator_setting() -> ::proc_macro2::TokenStream {
    ::quote::quote! {
        use ::std::alloc::System as FFISystem;
        #[global_allocator]
        static GLOBAL: FFISystem = FFISystem;
    }
}
