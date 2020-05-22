pub use error_code::{SystemError, PlainError, PosixError};

#[test]
fn it_works() {
    let error = PosixError::new(11);
    eprintln!("{}", error);

    let error = PosixError::last();
    eprintln!("{}", error);

    let error = PlainError::new(11);
    eprintln!("{}", error);

    let error = SystemError::new(13000);
    eprintln!("{}", error);

    let error = SystemError::last();
    eprintln!("{}", error);

    let error = PlainError::new(11);
    eprintln!("{}", error);

}

#[cfg(feature = "ufmt")]
#[test]
fn it_works_ufmt() {
    ufmt_stdio::init();

    let error = PosixError::new(11);
    ufmt_stdio::eprintln!("{}", error);

    let error = PosixError::new(i32::max_value());
    ufmt_stdio::eprintln!("{}", error);
}
