/// Generic error category, suitable for all environments.
///
/// In presence of OS, it means it identifies POSIX error codes.

#[derive(Copy, Clone)]
pub struct GenericCategory;

impl GenericCategory {
    #[inline]
    ///Access static instance
    pub fn new() -> Self {
        Self
    }
}

pub fn get_last_error() -> i32 {
    #[cfg(not(any(target_os = "wasi", target_os = "unknown")))]
    {
        extern {
            #[cfg(not(target_os = "dragonfly"))]
            #[cfg_attr(any(target_os = "macos",
                           target_os = "ios",
                           target_os = "freebsd"),
                       link_name = "__error")]
            #[cfg_attr(any(target_os = "openbsd",
                           target_os = "netbsd",
                           target_os = "bitrig",
                           target_os = "android"),
                       link_name = "__errno")]
            #[cfg_attr(target_os = "solaris",
                       link_name = "___errno")]
            #[cfg_attr(target_os = "linux",
                       link_name = "__errno_location")]
            #[cfg_attr(target_os = "windows",
                       link_name = "_errno")]
            fn errno_location() -> *mut i32;
        }

        return unsafe {
            *(errno_location())
        }
    }

    #[cfg(target_os = "wasi")]
    {
        extern {
            #[thread_local]
            #[link_name = "errno"]
            static mut libc_errno: i32;
        }

        return libc_errno;
    }

    #[cfg(target_os = "unknown")]
    {
        return 0;
    }
}

fn to_error(code: i32) -> alloc::borrow::Cow<'static, str> {
    #[cfg(any(windows, all(unix, not(target_env = "gnu"))))]
    extern "C" {
        ///Only GNU impl is thread unsafe
        fn strerror(code: i32) -> *const i8;
        fn strlen(text: *const i8) -> usize;
    }

    #[cfg(all(unix, target_env = "gnu"))]
    extern "C" {
        fn strerror_l(code: i32, locale: *mut i8) -> *const i8;
        fn strlen(text: *const i8) -> usize;
    }

    #[cfg(all(unix, target_env = "gnu"))]
    #[inline]
    unsafe fn strerror(code: i32) -> *const i8 {
        strerror_l(code, core::ptr::null_mut())
    }

    #[cfg(any(windows, unix))]
    {
        let err = unsafe {
            strerror(code)
        };

        if !err.is_null() {
            let err_slice = unsafe {
                core::slice::from_raw_parts(err as *const u8, strlen(err))
            };

            match core::str::from_utf8(err_slice) {
                Ok(res) => return res.into(),
                Err(_) => (),
            }
        }
    }

    alloc::borrow::Cow::Borrowed("Unknown error")
}

impl crate::Category for GenericCategory {
    fn name(&self) -> &'static str {
        "Generic"
    }

    fn message(&self, code: i32) -> alloc::borrow::Cow<'_, str> {
        to_error(code)
    }
}
