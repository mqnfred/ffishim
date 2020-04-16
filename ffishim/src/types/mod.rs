lazy_static! {
    pub(crate) static ref BEHAVIORS: Vec<Box<dyn crate::TypeBehavior>> = vec![
        Box::new(string::Behavior),
        Box::new(result::Behavior),
    ];
}

mod string;
pub mod result;
