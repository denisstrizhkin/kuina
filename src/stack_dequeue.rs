use core::slice;
use std::mem::MaybeUninit;

pub struct StackDequeue<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    start: usize,
    size: usize,
}

impl<T, const N: usize> StackDequeue<T, N> {
    pub fn new() -> Self {
        Self {
            data: [const { MaybeUninit::uninit() }; N],
            size: 0,
            start: 0,
        }
    }

    pub fn push_back(&mut self, value: T) {
        self.push_back_mut(value);
    }

    pub fn push_back_mut(&mut self, value: T) -> &mut T {
        assert!(self.size < N);
        let idx = (self.start + self.size) % N;
        self.size += 1;
        unsafe { self.data.get_unchecked_mut(idx).write(value) }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.size = self.size.checked_sub(1)?;
        let start = self.start;
        self.start = (self.start + 1) % N;
        unsafe { Some(self.data.get_unchecked(start).assume_init_read()) }
    }

    pub fn push_front(&mut self, value: T) {
        self.push_front_mut(value);
    }

    pub fn push_front_mut(&mut self, value: T) -> &mut T {
        assert!(self.size < N);
        self.size += 1;
        let start = self.start;
        self.start = self.start.checked_sub(1).unwrap_or(N - 1);
        unsafe { self.data.get_unchecked_mut(start).write(value) }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.size = self.size.checked_sub(1)?;
        let idx = (self.start + self.size) % N;
        unsafe { Some(self.data.get_unchecked(idx).assume_init_read()) }
    }

    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let mut deque = StackDequeue::<_, 5>::new();
    /// deque.push_back(0);
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// let expected = [0, 1, 2];
    /// let (front, back) = deque.as_slices();
    /// assert_eq!(&expected[..front.len()], front);
    /// assert_eq!(&expected[front.len()..], back);
    /// deque.push_front(10);
    /// deque.push_front(9);
    /// let expected = [9, 10, 0, 1, 2];
    /// let (front, back) = deque.as_slices();
    /// assert_eq!(&expected[..front.len()], front);
    /// assert_eq!(&expected[front.len()..], back);
    /// ```
    pub fn as_slices(&self) -> (&[T], &[T]) {
        let ptr1 = self.data[self.start..].as_ptr() as *const T;
        let len1 = (N - self.start).min(self.size);
        let ptr2 = self.data[(self.start + len1) % N..].as_ptr() as *const T;
        let len2 = self.size - len1;
        unsafe {
            (
                slice::from_raw_parts(ptr1, len1),
                slice::from_raw_parts(ptr2, len2),
            )
        }
    }
}
