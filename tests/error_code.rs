pub use error_code::{SystemError, PlainError, PosixError};

#[test]
fn it_works() {
    let error = PosixError::new(11);
    eprintln!("{:?}", error.to_string());
    eprintln!("{:?}", error);

    let error = PosixError::last();
    eprintln!("{}", error);

    let error = PlainError::new(11);
    eprintln!("{}", error);

    let error = SystemError::new(11);
    eprintln!("{:?}", error.to_string());

    let error = SystemError::last();
    eprintln!("{}", error);

    let error = PlainError::new(11);
    eprintln!("{}", error);

    let error = SystemError::unimplemented();
    eprintln!("{:?}", error.to_string());
}
