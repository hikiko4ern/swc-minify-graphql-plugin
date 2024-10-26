use core::str;
use std::{
    alloc::Allocator,
    ops::{Deref, RangeBounds},
    vec::Drain,
};

/// a wrapper over [`Vec<u8>`] that guarantees that the buffer content is a valid UTF-8 string
pub(crate) struct VecStr<A: Allocator>(Vec<u8, A>);

impl<A: Allocator> Deref for VecStr<A> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<A: Allocator> VecStr<A> {
    pub fn new_in(alloc: A) -> Self {
        Self(Vec::new_in(alloc))
    }

    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        Self(Vec::with_capacity_in(capacity, alloc))
    }

    #[cfg(test)]
    pub fn from_str_in(s: &str, alloc: A) -> Self {
        let len = s.len();
        let mut buf = Self::with_capacity_in(len, alloc);
        // SAFETY:
        // * `src` is valid for reads of `s.len()` bytes by virtue of being an allocated `&str`.
        // * `dst` is valid for writes of `s.len()` bytes as `String::with_capacity_in(s.len(), bump)`
        //   above guarantees that.
        // * Alignment is not relevant as `u8` has no alignment requirements.
        // * Source and destination ranges cannot overlap as we just reserved the destination
        //   range from the bump.
        unsafe { core::ptr::copy_nonoverlapping(s.as_ptr(), buf.0.as_mut_ptr(), len) };
        // SAFETY: We reserved sufficient capacity for the string above.
        // The elements at `0..len` were initialized by `copy_nonoverlapping` above.
        unsafe { buf.0.set_len(len) };
        buf
    }

    pub fn push(&mut self, char: u8) {
        self.0.push(char);
    }

    pub fn push_str<Str: AsRef<str>>(&mut self, str: Str) {
        self.0.extend_from_slice(str.as_ref().as_bytes());
    }

    pub fn drain<R>(&mut self, range: R) -> Drain<'_, u8, A>
    where
        R: RangeBounds<usize>,
    {
        self.0.drain(range)
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl<A: Allocator> AsRef<str> for VecStr<A> {
    fn as_ref(&self) -> &str {
        // SAFETY: vec is guaranteed to be a valid UTF-8 string,
        // since it is composed only from valid UTF-8 strings
        unsafe { str::from_utf8_unchecked(self) }
    }
}
