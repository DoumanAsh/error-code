use error_code::ErrorCode;

use core::mem;

#[cfg(target_pointer_width = "64")]
#[test]
fn size_check_64bit() {
    //On 64bit we suffer from alignment, but Rust optimizes enums quite well so ErrorCode benefits
    //of this optimization, letting its padding to be consumed by Result
    assert_eq!(mem::size_of::<ErrorCode>(), 16);
    //This optimization is enabled in latest rust compiler
    //assert_eq!(mem::size_of::<Result<bool, ErrorCode>>(), 16);
}

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
