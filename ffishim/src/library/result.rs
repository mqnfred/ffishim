impl<T> From<::std::result::Result<T, super::anyhow::Error>> for super::Result<T> {
    fn from(res: ::std::result::Result<T, super::anyhow::Error>) -> Self {
        match res {
            Ok(t) => Self::success(t),
            Err(err) => Self::error(err),
        }
    }
}

impl<T> super::Result<T> {
    pub fn success(payload: T) -> Self {
        Self {
            error: ::std::ptr::null_mut(),
            payload: Box::into_raw(Box::new(payload)),
        }
    }

    pub fn error(err: ::anyhow::Error) -> Self {
        Self{
            error: ::std::ffi::CString::new(err.to_string()).unwrap().into_raw(),
            payload: ::std::ptr::null_mut(),
        }
    }

    pub fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    pub fn from_raw(input: *mut Self) -> Self {
        unsafe { *Box::from_raw(input) }
    }
}

#[no_mangle]
pub extern "C" fn free_result(res: *mut super::Result<u64>) {
    if !res.is_null() {
        let res = *unsafe { Box::from_raw(res) };

        if !res.error.is_null() {
            unsafe { ::std::ffi::CString::from_raw(res.error) };
        }

        if !res.payload.is_null() {
            unsafe { Box::from_raw(res.payload) };
        }
    }
}
