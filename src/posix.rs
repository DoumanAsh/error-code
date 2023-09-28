use crate::{Category, MessageBuf, ErrorCode};
use crate::utils::write_message_buf;

use core::{ptr, ffi, str};

/// Posix error category, suitable for all environments.
///
/// In presence of OS, it means it identifies POSIX error codes.
pub static POSIX_CATEGORY: Category = Category {
    name: "PosixError",
    message,
    equivalent,
    is_would_block,
};

fn equivalent(code: ffi::c_int, other: &ErrorCode) -> bool {
    ptr::eq(&POSIX_CATEGORY, other.category()) && code == other.raw_code()
}

pub(crate) fn get_last_error() -> ffi::c_int {
    #[cfg(not(any(target_os = "wasi", target_os = "cloudabi", target_os = "unknown")))]
    {
        extern {
            #[cfg(not(target_os = "dragonfly"))]
            #[cfg_attr(any(target_os = "macos", target_os = "ios", target_os = "freebsd"), link_name = "__error")]
            #[cfg_attr(
                any(
                    target_os = "openbsd",
                    target_os = "netbsd",
                    target_os = "bitrig",
                    target_os = "android",
                    target_os = "espidf"
                ),
                link_name = "__errno"
            )]
            #[cfg_attr(
                any(target_os = "solaris", target_os = "illumos"),
                link_name = "___errno"
            )]
            #[cfg_attr(target_os = "haiku", link_name = "_errnop")]
            #[cfg_attr(
                any(target_os = "linux", target_os = "hurd", target_os = "redox"),
                link_name = "__errno_location"
            )]
            #[cfg_attr(target_os = "aix", link_name = "_Errno")]
            #[cfg_attr(target_os = "nto", link_name = "__get_errno_ptr")]
            #[cfg_attr(target_os = "windows", link_name = "_errno")]
            fn errno_location() -> *mut ffi::c_int;
        }

        return unsafe {
            *(errno_location())
        }
    }

    #[cfg(any(target_os = "cloudabi", target_os = "wasi"))]
    {
        extern {
            #[thread_local]
            static errno: ffi::c_int;
        }

        return errno;
    }

    #[cfg(target_os = "vxworks")]
    {
        extern "C" {
            pub fn errnoGet() -> ffi::c_int;
        }

        return unsafe {
            errnoGet();
        }
    }

    #[cfg(target_os = "unknown")]
    {
        return 0;
    }
}

pub(crate) fn message(code: ffi::c_int, out: &mut MessageBuf) -> &str {
    #[cfg(any(windows, all(unix, not(target_env = "gnu"))))]
    extern "C" {
        ///Only GNU impl is thread unsafe
        fn strerror(code: ffi::c_int) -> *const i8;
        fn strlen(text: *const i8) -> usize;
    }

    #[cfg(all(unix, target_env = "gnu"))]
    extern "C" {
        fn strerror_l(code: ffi::c_int, locale: *mut i8) -> *const i8;
        fn strlen(text: *const i8) -> usize;
    }

    #[cfg(all(unix, target_env = "gnu"))]
    #[inline]
    unsafe fn strerror(code: ffi::c_int) -> *const i8 {
        strerror_l(code, ptr::null_mut())
    }

    #[cfg(any(windows, unix))]
    {
        let err = unsafe {
            strerror(code)
        };

        if !err.is_null() {
            let err_len = unsafe {
                strlen(err)
            };

            debug_assert!(out.len() >= err_len);

            let err_slice = unsafe {
                ptr::copy_nonoverlapping(err as *const u8, out.as_mut_ptr() as *mut u8, err_len);
                core::slice::from_raw_parts(out.as_ptr() as *const u8, err_len)
            };

            match str::from_utf8(err_slice) {
                Ok(msg) => return msg,
                Err(_) => return write_message_buf(out, crate::FAIL_ERROR_FORMAT),
            };
        }
    }

    write_message_buf(out, crate::UNKNOWN_ERROR)
}

#[cfg(not(any(windows, unix)))]
pub(crate) const fn is_would_block(_: ffi::c_int) -> bool {
    false
}

#[cfg(any(windows, unix))]
pub(crate) const fn is_would_block(code: ffi::c_int) -> bool {
    code == libc::EWOULDBLOCK || code == libc::EAGAIN
}
