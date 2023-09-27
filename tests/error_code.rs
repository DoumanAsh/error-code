pub use error_code::ErrorCode;

#[test]
fn it_works() {
    let error = ErrorCode::new_posix(11);
    eprintln!("{:?}", error.to_string());
    eprintln!("{:?}", error);

    let error = ErrorCode::last_posix();
    eprintln!("{}", error);

    let error = ErrorCode::new_system(11);
    eprintln!("{:?}", error.to_string());

    let error = ErrorCode::last_system();
    eprintln!("{}", error);
}

#[test]
fn check_error_code_range() {
    for code in 0..=15999 {
        let error = ErrorCode::new_posix(code);
        eprintln!("{:?}", error.to_string());

        let error = ErrorCode::new_system(code);
        eprintln!("{:?}", error.to_string());
    }
}
