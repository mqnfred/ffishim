impl<T> super::FFIVec<T> {
    pub fn from(vec: Vec<T>) -> Self {
        let (ptr, len, cap) = vec.into_raw_parts();
        Self{ptr, len, cap}
    }

    pub fn into_vec(array: Self) -> Vec<T> {
        unsafe { Vec::from_raw_parts(array.ptr, array.len, array.cap) }
    }
}
