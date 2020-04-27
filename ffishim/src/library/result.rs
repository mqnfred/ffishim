impl<T> From<::std::result::Result<T, super::anyhow::Error>> for super::FFIResult<T> {
    fn from(res: ::std::result::Result<T, super::anyhow::Error>) -> Self {
        match res {
            Ok(t) => Self::success(t),
            Err(err) => Self::error(err),
        }
    }
}

impl<T> super::FFIResult<T> {
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
}

/// No function declared in the ffishim library will be carried by the generated shared object
/// library. The `free_result` function needs to be declared *inside* the generated crate. For this
/// purpose, the `derive_ffishim` crate calls `free_result_function` to get the definition and
/// insert it into the generated crate.
pub fn free_result_function() -> ::syn::ItemFn {
    ::syn::parse_quote! {
        #[no_mangle]
        pub extern "C" fn free_result(res: *mut ::ffishim::library::FFIResult<u64>) {
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
    }
}
