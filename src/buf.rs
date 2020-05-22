use core::{mem, slice, ptr, cmp};

///Stack based string.
pub struct StrBuf<T: Sized> {
    inner: mem::MaybeUninit<T>,
    cursor: u8, //number of bytes written
}

impl<S: Sized> StrBuf<S> {
    #[inline]
    ///Creates new instance
    pub const fn new() -> Self {
        Self {
            inner: mem::MaybeUninit::uninit(),
            cursor: 0,
        }
    }

    #[inline]
    ///Returns pointer  to the beginning of underlying buffer
    pub const fn as_ptr(&self) -> *const u8 {
        &self.inner as *const _ as *const u8
    }

    #[inline]
    ///Returns number of bytes left (not written yet)
    pub const fn remaining(&self) -> usize {
        Self::capacity() - self.cursor as usize
    }

    #[inline]
    ///Returns slice to already written data.
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.as_ptr(), self.cursor as usize)
        }
    }

    #[inline]
    ///Returns mutable slice to already written data.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self.as_ptr() as *mut u8, self.cursor as usize)
        }
    }

    #[inline]
    ///Shortens the buffer.
    ///
    ///Does nothing if new `cursor` is after current position.
    pub fn truncate(&mut self, cursor: u8) {
        if cursor < self.cursor {
            self.cursor = cursor
        }
    }

    #[inline]
    ///Returns buffer overall capacity.
    pub const fn capacity() -> usize {
        mem::size_of::<S>()
    }

    #[inline]
    ///Returns number of bytes written.
    pub const fn len(&self) -> usize {
        self.cursor as usize
    }

    #[inline]
    ///Appends given string, truncating on overflow
    pub fn push_str(&mut self, text: &str) {
        let size = cmp::min(text.len(), self.remaining());
        unsafe {
            ptr::copy_nonoverlapping(text.as_ptr(), self.as_ptr().offset(self.cursor as isize) as *mut u8, size);
        }
        self.cursor = self.cursor.saturating_add(size as u8);
    }

    #[inline(always)]
    ///Access str from underlying storage
    ///
    ///Returns empty if nothing has been written into buffer yet.
    pub fn as_str(&self) -> &str {
        unsafe {
            let slice = core::slice::from_raw_parts(self.as_ptr(), self.len());
            core::str::from_utf8_unchecked(slice)
        }
    }
}

impl<S: Sized> AsRef<str> for StrBuf<S> {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<S: Sized> core::fmt::Write for StrBuf<S> {
    #[inline(always)]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.push_str(s);
        Ok(())
    }
}

impl<S: Sized> core::fmt::Display for StrBuf<S> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<S: Sized> core::fmt::Debug for StrBuf<S> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}
