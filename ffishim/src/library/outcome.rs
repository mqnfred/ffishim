impl<T> From<Result<T, super::anyhow::Error>> for super::Outcome<T> {
    fn from(res: Result<T, super::anyhow::Error>) -> Self {
        match res {
            Ok(t) => Self::success(t),
            Err(err) => Self::error(err),
        }
    }
}

impl<T> super::Outcome<T> {
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
pub extern "C" fn free_outcome(outcome: *mut super::Outcome<u64>) {
    if !outcome.is_null() {
        let outcome = *unsafe { Box::from_raw(outcome) };

        if !outcome.error.is_null() {
            unsafe { ::std::ffi::CString::from_raw(outcome.error) };
        }

        if !outcome.payload.is_null() {
            unsafe { Box::from_raw(outcome.payload) };
        }
    }
}
