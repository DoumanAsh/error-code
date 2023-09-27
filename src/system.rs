use crate::{Category, ErrorCode};
#[cfg(not(windows))]
use crate::posix::{message, is_would_block};
#[cfg(not(windows))]
pub(crate) use crate::posix::get_last_error;

use core::{ptr, ffi};

/// System error category, suitable for all environments.
///
/// On UNIX system it is equivalent of [Posix](struct.PosixCategory.html)
///
/// On Windows it uses winapi error functions
pub static SYSTEM_CATEGORY: Category = Category {
    name: "OSError",
    message,
    equivalent,
    is_would_block,
};

fn equivalent(code: ffi::c_int, other: &ErrorCode) -> bool {
    ptr::eq(&SYSTEM_CATEGORY, other.category()) && code == other.raw_code()
}

#[cfg(windows)]
#[inline]
pub(crate) fn get_last_error() -> ffi::c_int {
    unsafe {
        GetLastError() as ffi::c_int
   }
}

#[cfg(windows)]
fn message(code: ffi::c_int, out: &mut crate::MessageBuf) -> &str {
    use crate::MESSAGE_BUF_SIZE;
    use core::{slice, mem};

    const CP_UTF8: u32 = 65001;
    const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 0x00001000;
    const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 0x00000200;
    const FMT_FLAGS: u32 = FORMAT_MESSAGE_IGNORE_INSERTS | FORMAT_MESSAGE_FROM_SYSTEM;

    let mut buff = [mem::MaybeUninit::<u16>::uninit(); MESSAGE_BUF_SIZE * 2];
    let mut len: u32 = unsafe {
        FormatMessageW(FMT_FLAGS, ptr::null(), code as u32, 0, buff.as_mut_ptr() as *mut u16, buff.len() as _, ptr::null_mut())
    };

    if len == 0 {
        match get_last_error() {
            //Buffer doesn't have enough space
            //But it is completely written so we'll take what we can
            122 => len = buff.len() as u32,
            //System cannot find specified error code
            317 => return crate::posix::message(code, out),
            _ => return crate::FAIL_ERROR_FORMAT
        }
    }

    let res = unsafe {
        WideCharToMultiByte(CP_UTF8, 0,
                            buff.as_ptr() as _, len as _,
                            out.as_mut_ptr() as *mut i8, out.len() as _,
                            ptr::null(), ptr::null_mut())
    };

    match res {
        0 => match get_last_error() {
            122 => "<Truncated>",
            _ => crate::FAIL_ERROR_FORMAT,
        }
        len => {
            let out = unsafe {
                slice::from_raw_parts(out.as_ptr() as *const u8, len as _)
            };
            //It seems WinAPI always supposed to have at the end null char.
            //But just to be safe let's check for it and only then remove.
            let actual_len = if let Some(null_idx) = out.iter().position(|b| *b == b'\0' || *b == b'\r') {
                null_idx
            } else {
                len as usize
            };

            unsafe {
                core::str::from_utf8_unchecked(
                    slice::from_raw_parts(out.as_ptr(), actual_len)
                )
            }
        }
    }
}

#[cfg(windows)]
#[inline]
const fn is_would_block(code: ffi::c_int) -> bool {
    code == 10035 || crate::posix::is_would_block(code)
}

#[cfg(windows)]
extern "system" {
    fn GetLastError() -> u32;
    fn FormatMessageW(dwFlags: u32, lpSource: *const u8, dwMessageId: u32, dwLanguageId: u32, lpBuffer: *mut u16, nSize: u32, Arguments: *mut i8) -> u32;
    fn WideCharToMultiByte(page: ffi::c_uint, flags: ffi::c_ulong, wide_str: *const u16, wide_str_len: ffi::c_int, multi_str: *mut i8, multi_str_len: ffi::c_int, default_char: *const i8, used_default_char: *mut bool) -> ffi::c_int;
}
