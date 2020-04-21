impl<T> super::Array<T> {
    pub fn from(vec: Vec<T>) -> Self {
        let (ptr, len, cap) = vec.into_raw_parts();
        Self{ptr, len, cap}
    }

    pub fn into_vec(array: Self) -> Vec<T> {
        unsafe { Vec::from_raw_parts(array.ptr, array.len, array.cap) }
    }
}

impl<T> super::Array<T> {
    pub fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    pub fn from_raw(input: *mut Self) -> Self {
        unsafe { *Box::from_raw(input) }
    }
}
