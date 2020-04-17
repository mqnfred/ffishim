lazy_static! {
    pub(crate) static ref BEHAVIORS: Vec<Box<dyn crate::TypeBehavior>> = vec![
        Box::new(numbers::Behavior),
        Box::new(result::Behavior),
        Box::new(string::Behavior),
    ];
}

mod numbers;
pub mod result;
mod string;
