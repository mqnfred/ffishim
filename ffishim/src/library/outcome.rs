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
            errorcode: 0,
            message: ::std::ptr::null_mut(),
            payload: Box::into_raw(Box::new(payload)),
        }
    }

    pub fn error(err: ::anyhow::Error) -> Self {
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
