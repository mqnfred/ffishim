static mut DEFINED: Option<::std::collections::HashSet<String>> = None;

pub unsafe fn defined_once<T>(name: &str, item: T) -> Option<T> {
    if get(name) {
        None
    } else {
        set(name);
        Some(item)
    }
}

unsafe fn set(name: &str) {
    if let Some(defined) = DEFINED.as_mut() {
        defined.insert(name.to_owned());
    } else {
        DEFINED = Some(::std::collections::HashSet::new());
    }
    DEFINED.as_mut().unwrap().insert(name.to_owned());
}

unsafe fn get(name: &str) -> bool {
    if let Some(defined) = DEFINED.as_mut() {
        defined.get(name).is_some()
    } else {
        false
    }
}
