use ::syn::*;

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
