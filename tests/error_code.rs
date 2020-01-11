pub use error_code::ErrorCode;

#[test]
fn it_works() {
    let error = ErrorCode::new_generic(11);
    eprintln!("{}", error);

    let error = ErrorCode::last_generic();
    eprintln!("{}", error);
}

#[cfg(feature = "ufmt")]
#[test]
fn it_works_ufmt() {
    ufmt_stdio::init();

    let error = ErrorCode::new_generic(i32::max_value());
    ufmt_stdio::eprintln!("{}", error);
}
