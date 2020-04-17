impl<T, E: ::std::error::Error> From<Result<T, E>> for super::Outcome<T> {
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

impl<T> super::Outcome<T> {
    pub fn success(payload: T) -> Self {
        Self {
            errorcode: 0,
            message: ::std::ptr::null_mut(),
            payload: Box::into_raw(Box::new(payload)),
        }
    }

    pub fn error(err: impl ::std::error::Error) -> Self {
        Self{
            errorcode: 1,
            message: ::std::ffi::CString::new(err.to_string()).unwrap().into_raw(),
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
