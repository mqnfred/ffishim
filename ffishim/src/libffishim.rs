impl<T> From<Vec<T>> for crate::Array<T> {
    fn from(vec: Vec<T>) -> Self {
        let (ptr, len, cap) = vec.into_raw_parts();
        Self{ptr, len: len as u64, cap: cap as u64}
    }
}

impl<T> Into<Vec<T>> for crate::Array<T> {
    fn into(self) -> Vec<T> {
        unsafe { Vec::from_raw_parts(self.ptr, self.len as usize, self.cap as usize) }
    }
}

impl<T> Drop for crate::Array<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { Vec::from_raw_parts(self.ptr, self.len as usize, self.cap as usize) };
        }
    }
}

impl<T> crate::Array<T> {
    pub fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    pub fn from_raw(input: *mut Self) -> Self {
        unsafe { *Box::from_raw(input) }
    }
}

impl<T, E: ::std::error::Error> From<Result<T, E>> for crate::Outcome<T> {
    fn from(res: Result<T, E>) -> Self {
        match res {
            Ok(t) => Self::success(t),
            Err(err) => Self {
                errorcode: 1,
                message: ::std::ffi::CString::new(err.to_string()).unwrap().into_raw(),
                payload: ::std::ptr::null_mut(),
            },
        }
    }
}

impl<T> crate::Outcome<T> {
    pub fn success(payload: T) -> Self {
        Self {
            errorcode: 0,
            message: ::std::ptr::null_mut(),
            payload: Box::into_raw(Box::new(payload)),
        }
    }

    pub fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    pub fn from_raw(input: *mut Self) -> Self {
        unsafe { *Box::from_raw(input) }
    }
}
