//! Error code library provides generic errno/winapi error wrapper
//!
//! User can define own `Category` if you want to create new error wrapper.
//!
//! ## Usage
//!
//! ```rust
//! use error_code::ErrorCode;
//!
//! use std::fs::File;
//!
//! File::open("non_existing");
//! println!("{}", ErrorCode::last_system());
//! ```

#![no_std]
#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

#[cfg(feature = "std")]
extern crate std;

use core::{mem, hash, fmt};

///Text to return when cannot map error
pub const UNKNOWN_ERROR: &str = "Unknown error";
///Text to return when error fails to be converted into utf-8
pub const FAIL_ERROR_FORMAT: &str = "Failed to format error into utf-8";

const MESSAGE_BUF_SIZE: usize = 256;
///Type alias for buffer to hold error code description.
pub type MessageBuf = [mem::MaybeUninit<u8>; MESSAGE_BUF_SIZE];

pub mod defs;
pub mod types;
mod utils;
mod posix;
pub use posix::POSIX_CATEGORY;
mod system;
pub use system::SYSTEM_CATEGORY;

///Interface for error category
///
///It is implemented as pointers in order to avoid generics or overhead of fat pointers.
pub struct Category {
    ///Category name
    pub name: &'static str,
    ///Maps error code and writes descriptive error message accordingly.
    ///
    ///In case of insufficient buffer, prefer to truncate message or just don't write big ass message.
    ///
    ///In case of error, just write generic name.
    ///
    ///Returns formatted message as string.
    pub message: fn(types::c_int, &mut MessageBuf) -> &str,
    ///Checks whether error code is equivalent to another one.
    ///
    ///## Args:
    ///
    ///- Raw error code, belonging to this category
    ///- Another error code being compared against this category.
    ///
    ///## Recommendation
    ///
    ///Generally error code is equal if it belongs to the same category (use `ptr::eq` to compare
    ///pointers to `Category`) and raw error codes are equal.
    pub equivalent: fn(types::c_int, &ErrorCode) -> bool,
    ///Returns `true` if supplied error code indicates WouldBlock like error.
    ///
    ///This should `true` only for errors that indicate operation can be re-tried later.
    pub is_would_block: fn(types::c_int) -> bool,
}

#[derive(Copy, Clone)]
///Describes error code of particular category.
pub struct ErrorCode {
    code: types::c_int,
    category: &'static Category
}

impl ErrorCode {
    #[inline]
    ///Initializes error code with provided category
    pub const fn new(code: types::c_int, category: &'static Category) -> Self {
        Self {
            code,
            category,
        }
    }

    #[inline(always)]
    ///Creates new POSIX error code.
    pub fn new_posix(code: types::c_int) -> Self {
        Self::new(code, &POSIX_CATEGORY)
    }

    #[inline(always)]
    ///Creates new System error code.
    pub fn new_system(code: types::c_int) -> Self {
        Self::new(code, &SYSTEM_CATEGORY)
    }

    #[inline]
    ///Gets last POSIX error
    pub fn last_posix() -> Self {
        Self::new_posix(posix::get_last_error())
    }

    #[inline]
    ///Gets last System error
    pub fn last_system() -> Self {
        Self::new_system(system::get_last_error())
    }

    #[inline(always)]
    ///Gets raw error code.
    pub const fn raw_code(&self) -> types::c_int {
        self.code
    }

    #[inline(always)]
    ///Gets reference to underlying Category.
    pub const fn category(&self) -> &'static Category {
        self.category
    }

    #[inline(always)]
    ///Returns `true` if underlying error indicates operation can or should be re-tried at later date.
    pub fn is_would_block(&self) -> bool {
        (self.category.is_would_block)(self.code)
    }
}

impl PartialEq for ErrorCode {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.category.equivalent)(self.code, other)
    }
}

impl Eq for ErrorCode {}

impl hash::Hash for ErrorCode {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}

impl fmt::Debug for ErrorCode {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_fmt(format_args!("{}({})", self.category.name, self.code))
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = [mem::MaybeUninit::uninit(); MESSAGE_BUF_SIZE];
        let message = (self.category.message)(self.code, &mut out);
        fmt::Debug::fmt(self, fmt)?;
        fmt.write_str(": ")?;
        fmt.write_str(message)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ErrorCode {}

#[cfg(feature = "std")]
impl From<std::io::Error> for ErrorCode {
    #[inline]
    fn from(err: std::io::Error) -> Self {
        match err.raw_os_error() {
            Some(err) => Self::new_posix(err),
            None => Self::new_posix(-1),
        }
    }
}
