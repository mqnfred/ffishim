impl<T> From<Vec<T>> for super::Array<T> {
    fn from(vec: Vec<T>) -> Self {
        let (ptr, len, cap) = vec.into_raw_parts();
        Self{ptr, len: len as u64, cap: cap as u64}
    }
}

impl<T> Into<Vec<T>> for super::Array<T> {
    fn into(self) -> Vec<T> {
        unsafe { Vec::from_raw_parts(self.ptr, self.len as usize, self.cap as usize) }
    }
}

impl<T> Drop for super::Array<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { Vec::from_raw_parts(self.ptr, self.len as usize, self.cap as usize) };
        }
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
