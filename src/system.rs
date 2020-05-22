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
#[inline(always)]
pub fn to_error(code: i32) -> crate::Str {
    use core::fmt::Write;

    const FORMAT_MESSAGE_ARGUMENT_ARRAY: u32 = 0x00002000;
    const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 0x00001000;
    const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 0x00000200;

    const BUF_SIZE: usize = 256;
    const FMT_FLAGS: u32 = FORMAT_MESSAGE_IGNORE_INSERTS | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_ARGUMENT_ARRAY;
    let mut res = crate::Str::new();
    let mut buff: [u16; BUF_SIZE] = [0; BUF_SIZE];

    let num_chars: u32 = unsafe { FormatMessageW(FMT_FLAGS,
                                                 core::ptr::null(), code as u32,
                                                 0, buff.as_mut_ptr(),
                                                 BUF_SIZE as u32, core::ptr::null_mut()) };

    if num_chars == 0 {
        match get_last_error() {
            //Insufficient memory
            122 => for ch in core::char::decode_utf16(buff.iter().cloned()).map(|r| r.unwrap_or(core::char::REPLACEMENT_CHARACTER)) {
                let _ = res.write_char(ch);
            },
            _ => res.push_str(crate::FAIL_FORMAT),
        }
    } else {
        let buff = &buff[..num_chars as usize-2];
        for ch in core::char::decode_utf16(buff.iter().cloned()).map(|r| r.unwrap_or(core::char::REPLACEMENT_CHARACTER)) {
            let _ = res.write_char(ch);
        }
    }

    res
}

impl crate::Category for SystemCategory {
    const NAME: &'static str = "OS error";

    #[inline]
    fn message<'a>(code: i32) -> crate::Str {
        to_error(code)
    }
}
