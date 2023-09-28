#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[cfg(not(any(windows, unix)))]
pub type c_int = i32;
#[cfg(any(windows, unix))]
pub use libc::c_int;

#[cfg(not(any(windows, unix)))]
pub type c_uint = u32;
#[cfg(any(windows, unix))]
pub use libc::c_uint;

#[cfg(not(any(windows, unix)))]
pub type c_ulong = u32;
#[cfg(any(windows, unix))]
pub use libc::c_ulong;
