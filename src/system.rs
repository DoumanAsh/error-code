/// System error category, suitable for all environments.
///
/// On UNIX system it is equivalent of [Posix](struct.PosixCategory.html)
///
/// On Windows it uses winapi error functions
pub struct SystemCategory;

#[cfg(not(windows))]
use crate::posix::to_error;
#[cfg(not(windows))]
pub use crate::posix::get_last_error;

#[cfg(windows)]
extern "system" {
    fn GetLastError() -> u32;
    fn FormatMessageW(dwFlags: u32, lpSource: *const u8, dwMessageId: u32, dwLanguageId: u32, lpBuffer: *mut u16, nSize: u32, Arguments: *mut i8) -> u32;
}

#[cfg(windows)]
#[inline]
pub fn get_last_error() -> i32 {
    unsafe {
        GetLastError() as i32
    }
}

#[cfg(windows)]
pub fn to_error<'a>(code: i32) -> alloc::borrow::Cow<'a, str> {
    const FORMAT_MESSAGE_ARGUMENT_ARRAY: u32 = 0x00002000;
    const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 0x00001000;
    const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 0x00000200;

    const BUF_SIZE: usize = 512;
    const FMT_FLAGS: u32 = FORMAT_MESSAGE_IGNORE_INSERTS | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_ARGUMENT_ARRAY;
    let mut buff: [u16; BUF_SIZE] = [0; BUF_SIZE];

    let num_chars: u32 = unsafe { FormatMessageW(FMT_FLAGS,
                                                 core::ptr::null(), code as u32,
                                                 0, buff.as_mut_ptr(),
                                                 BUF_SIZE as u32, core::ptr::null_mut()) };

    if num_chars == 0 {
        match get_last_error() {
            122 => alloc::string::String::from_utf16_lossy(&buff).into(), //Insufficient memory
            _ => alloc::borrow::Cow::Borrowed(crate::UNKNOWN_ERROR),
        }
    } else {
        alloc::string::String::from_utf16_lossy(&buff[..num_chars as usize-2]).into()
    }
}

impl crate::Category for SystemCategory {
    const NAME: &'static str = "OS error";

    #[inline]
    fn message<'a>(code: i32) -> alloc::borrow::Cow<'a, str> {
        to_error(code)
    }
}
