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
        assert!(self.size < N);
        self.data[self.size].write(value);
        self.size += 1;
    }

    pub fn push_unchecked(&mut self, value: T) {
        debug_assert!(self.size < N);
        self.data[self.size].write(value);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.size = self.size.checked_sub(1)?;
        unsafe { Some(self.data[self.size].assume_init_read()) }
    }
}

impl<T, const N: usize> Drop for StackVec<T, N> {
    fn drop(&mut self) {
        for item in self.data[..self.size].iter_mut() {
            unsafe { item.assume_init_drop() }
        }
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

impl<T, const N: usize> IntoIterator for StackVec<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            index: 0,
            vec: self,
        }
    }
}

pub struct IntoIter<T, const N: usize> {
    index: usize,
    vec: StackVec<T, N>,
}

impl<T, const N: usize> Drop for IntoIter<T, N> {
    fn drop(&mut self) {
        let len = self.vec.data.len();
        self.vec.size = 0;
        for item in self.vec.data[self.index..len].iter_mut() {
            unsafe { item.assume_init_drop() }
        }
    }
}

impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.index < self.vec.len()).then(|| {
            let index = self.index;
            self.index += 1;
            unsafe { self.vec.data[index].assume_init_read() }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.vec.len() - self.index, Some(self.vec.len()))
    }
}

impl<T, const N: usize> ExactSizeIterator for IntoIter<T, N> {}
