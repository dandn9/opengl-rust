pub fn to_c_str(str: &str) -> std::ffi::CString {
    std::ffi::CString::new(str.as_bytes()).unwrap()
}
