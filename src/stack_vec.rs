use std::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    slice,
};

pub struct StackVec<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    size: usize,
}

impl<T, const N: usize> Default for StackVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> StackVec<T, N> {
    pub fn new() -> Self {
        Self {
            data: [const { MaybeUninit::uninit() }; N],
            size: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        assert_ne!(self.size, N, "StackVec is at max size");
        self.data[self.size].write(value);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.size = self.size.checked_sub(1)?;
        unsafe { Some(self.data[self.size].assume_init_read()) }
    }
}

impl<T, const N: usize> From<[T; N]> for StackVec<T, N> {
    fn from(value: [T; N]) -> Self {
        Self {
            data: value.map(MaybeUninit::new),
            size: N,
        }
    }
}

impl<T, const N: usize> Deref for StackVec<T, N> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        let ptr = self.data.as_ptr() as *const T;
        unsafe { slice::from_raw_parts(ptr, self.size) }
    }
}

impl<T, const N: usize> DerefMut for StackVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = self.data.as_ptr() as *mut T;
        unsafe { slice::from_raw_parts_mut(ptr, self.size) }
    }
}

impl<T, const N: usize> Drop for StackVec<T, N> {
    fn drop(&mut self) {
        for item in self.data[..self.size].iter_mut() {
            unsafe { item.assume_init_drop() }
        }
    }
}
