use crate::MessageBuf;

use core::{ptr, slice};

pub(crate) fn write_message_buf<'a>(out: &'a mut MessageBuf, text: &str) -> &'a str {
    debug_assert!(text.len() <= out.len());
    unsafe {
        ptr::copy_nonoverlapping(text.as_ptr(), out.as_mut_ptr() as *mut u8, text.len());
        core::str::from_utf8_unchecked(
            slice::from_raw_parts(out.as_ptr() as *const u8, text.len())
        )
    }
}
