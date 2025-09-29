use core::slice;
use std::{
    fmt,
    mem::{self, MaybeUninit},
    ops::{Index, IndexMut},
};

pub struct StackDequeue<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    start: usize,
    size: usize,
}

impl<T, const N: usize> StackDequeue<T, N> {
    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let deq = StackDequeue::<u32, 5>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            data: [const { MaybeUninit::uninit() }; N],
            size: 0,
            start: 0,
        }
    }

    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let mut deq = StackDequeue::<_, 2>::new();
    /// assert_eq!(deq.len(), 0);
    /// deq.push_back(1);
    /// assert_eq!(deq.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.size
    }

    fn get_idx(&self, index: usize) -> usize {
        let index = self.start + index;
        if index < N { index } else { index - N }
    }

    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let mut deq = StackDequeue::<_, 4>::new();
    /// deq.push_back(3);
    /// deq.push_back(4);
    /// deq.push_back(5);
    /// deq.push_back(6);
    /// assert_eq!(deq.get(1), Some(&4));
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        (index < self.size).then(|| unsafe { self.data.get_unchecked(index).assume_init_ref() })
    }

    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let mut deq = StackDequeue::<_, 4>::new();
    /// deq.push_back(3);
    /// deq.push_back(4);
    /// deq.push_back(5);
    /// deq.push_back(6);
    /// assert_eq!(deq[1], 4);
    /// if let Some(elem) = deq.get_mut(1) {
    ///     *elem = 7;
    /// }
    /// assert_eq!(deq[1], 7);
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        (index < self.size).then(|| unsafe { self.data.get_unchecked_mut(index).assume_init_mut() })
    }

    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let mut deq = StackDequeue::<_, 2>::new();
    /// assert_eq!(deq.front(), None);
    /// deq.push_back(1);
    /// deq.push_back(2);
    /// assert_eq!(deq.front(), Some(&1));
    /// ```
    pub fn front(&self) -> Option<&T> {
        self.get(0)
    }

    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let mut deq = StackDequeue::<_, 2>::new();
    /// assert_eq!(deq.front_mut(), None);
    /// deq.push_back(1);
    /// deq.push_back(2);
    /// match deq.front_mut() {
    ///     Some(x) => *x = 9,
    ///     None => (),
    /// }
    /// assert_eq!(deq.front(), Some(&9));
    /// ```
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.get_mut(0)
    }

    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let mut deq = StackDequeue::<_, 2>::new();
    /// assert_eq!(deq.back(), None);
    /// deq.push_back(1);
    /// deq.push_back(2);
    /// assert_eq!(deq.back(), Some(&2));
    /// ```
    pub fn back(&self) -> Option<&T> {
        self.get(self.size.wrapping_sub(1))
    }

    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let mut deq = StackDequeue::<_, 4>::new();
    /// assert_eq!(deq.back(), None);
    /// deq.push_back(1);
    /// deq.push_back(2);
    /// match deq.back_mut() {
    ///     Some(x) => *x = 9,
    ///     None => (),
    /// }
    /// assert_eq!(deq.back(), Some(&9));
    /// ```
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.get_mut(self.size.wrapping_sub(1))
    }

    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let mut deq = StackDequeue::<_, 2>::new();
    /// deq.push_back(1);
    /// deq.push_back(3);
    /// assert_eq!(3, *deq.back().unwrap());
    /// ```
    pub fn push_back(&mut self, value: T) {
        self.push_back_mut(value);
    }

    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let mut deq = StackDequeue::<_, 3>::new();
    /// deq.push_back(1);
    /// deq.push_back(2);
    /// let x = deq.push_back_mut(9);
    /// *x += 1;
    /// assert_eq!(deq.back(), Some(&10));
    /// ```
    pub fn push_back_mut(&mut self, value: T) -> &mut T {
        assert!(self.size < N);
        let idx = self.get_idx(self.size);
        self.size += 1;
        unsafe { self.data.get_unchecked_mut(idx).write(value) }
    }

    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let mut deq = StackDequeue::<_, 2>::new();
    /// deq.push_back(1);
    /// deq.push_back(2);
    /// assert_eq!(deq.pop_front(), Some(1));
    /// assert_eq!(deq.pop_front(), Some(2));
    /// assert_eq!(deq.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        self.size = self.size.checked_sub(1)?;
        let start = self.start;
        self.start = self.get_idx(1);
        unsafe { Some(self.data.get_unchecked(start).assume_init_read()) }
    }

    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let mut deq = StackDequeue::<_, 2>::new();
    /// deq.push_front(1);
    /// deq.push_front(2);
    /// assert_eq!(deq.front(), Some(&2));
    /// ```
    pub fn push_front(&mut self, value: T) {
        self.push_front_mut(value);
    }

    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let mut deq = StackDequeue::<_, 3>::new();
    /// deq.push_back(1);
    /// deq.push_back(2);
    /// let x = deq.push_front_mut(8);
    /// *x -= 1;
    /// assert_eq!(deq.front(), Some(&7));
    /// ```
    pub fn push_front_mut(&mut self, value: T) -> &mut T {
        assert!(self.size < N);
        self.size += 1;
        let start = self.start;
        self.start = self.start.checked_sub(1).unwrap_or(N - 1);
        unsafe { self.data.get_unchecked_mut(start).write(value) }
    }

    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let mut deq = StackDequeue::<_, 2>::new();
    /// assert_eq!(deq.pop_back(), None);
    /// deq.push_back(1);
    /// deq.push_back(3);
    /// assert_eq!(deq.pop_back(), Some(3));
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        self.size = self.size.checked_sub(1)?;
        let idx = self.get_idx(self.size);
        unsafe { Some(self.data.get_unchecked(idx).assume_init_read()) }
    }

    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let mut deq = StackDequeue::<_, 5>::new();
    /// deq.push_back(0);
    /// deq.push_back(1);
    /// deq.push_back(2);
    /// let expected = [0, 1, 2];
    /// let (front, back) = deq.as_slices();
    /// assert_eq!(&expected[..front.len()], front);
    /// assert_eq!(&expected[front.len()..], back);
    /// deq.push_front(10);
    /// deq.push_front(9);
    /// let expected = [9, 10, 0, 1, 2];
    /// let (front, back) = deq.as_slices();
    /// assert_eq!(&expected[..front.len()], front);
    /// assert_eq!(&expected[front.len()..], back);
    /// ```
    pub fn as_slices(&self) -> (&[T], &[T]) {
        let ptr1 = self.data[self.start..].as_ptr() as *const T;
        let len1 = (N - self.start).min(self.size);
        let ptr2 = self.data[self.get_idx(self.start + len1)..].as_ptr() as *const T;
        let len2 = self.size - len1;
        unsafe {
            (
                slice::from_raw_parts(ptr1, len1),
                slice::from_raw_parts(ptr2, len2),
            )
        }
    }

    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let mut deq = StackDequeue::<_, 4>::new();
    /// deq.push_back(0);
    /// deq.push_back(1);
    /// deq.push_front(10);
    /// deq.push_front(9);
    /// let mut update_nth = |index: usize, val: u32| {
    ///     let (front, back) = deq.as_mut_slices();
    ///     if index > front.len() - 1 {
    ///         back[index - front.len()] = val;
    ///     } else {
    ///         front[index] = val;
    ///     }
    /// };
    /// update_nth(0, 42);
    /// update_nth(2, 24);
    /// assert_eq!(deq, [42, 10, 24, 1]);
    /// ```  
    pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
        let ptr1 = self.data[self.start..].as_ptr() as *mut T;
        let len1 = (N - self.start).min(self.size);
        let ptr2 = self.data[self.get_idx(self.start + len1)..].as_ptr() as *mut T;
        let len2 = self.size - len1;
        unsafe {
            (
                slice::from_raw_parts_mut(ptr1, len1),
                slice::from_raw_parts_mut(ptr2, len2),
            )
        }
    }

    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let mut deq = StackDequeue::<_, 3>::new();
    /// deq.push_back(5);
    /// deq.push_back(3);
    /// deq.push_back(4);
    /// let b: &[_] = &[&5, &3, &4];
    /// let c: Vec<&i32> = deq.iter().collect();
    /// assert_eq!(&c[..], b);
    /// ```
    pub fn iter<'a>(&'a self) -> Iter<'a, T, N> {
        self.into_iter()
    }

    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let mut deq = StackDequeue::<_, 3>::new();
    /// deq.push_back(5);
    /// deq.push_back(3);
    /// deq.push_back(4);
    /// for num in deq.iter_mut() {
    ///     *num = *num - 2;
    /// }
    /// let b: &[_] = &[&mut 3, &mut 1, &mut 2];
    /// assert_eq!(&deq.iter_mut().collect::<Vec<&mut i32>>()[..], b);
    /// ```
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T, N> {
        self.into_iter()
    }
}

impl<T, const N: usize> From<[T; N]> for StackDequeue<T, N> {
    /// ```
    /// use kuina::stack_dequeue::StackDequeue;
    /// let deq1 = StackDequeue::from([1, 2, 3, 4]);
    /// let deq2: StackDequeue<_, _> = [1, 2, 3, 4].into();
    /// assert_eq!(deq1, deq2);
    /// ```
    fn from(value: [T; N]) -> Self {
        Self {
            data: value.map(MaybeUninit::new),
            start: 0,
            size: N,
        }
    }
}

pub struct IntoIter<T, const N: usize>(StackDequeue<T, N>);

impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.size, Some(self.0.size))
    }
}

impl<T, const N: usize> DoubleEndedIterator for IntoIter<T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<T, const N: usize> ExactSizeIterator for IntoIter<T, N> {}

impl<T, const N: usize> IntoIterator for StackDequeue<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct Iter<'a, T, const N: usize> {
    a: slice::Iter<'a, T>,
    b: slice::Iter<'a, T>,
}

impl<'a, T, const N: usize> Iterator for Iter<'a, T, N> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.a.next() {
            Some(value) => Some(value),
            None => {
                mem::swap(&mut self.a, &mut self.b);
                self.a.next()
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.a.len() + self.b.len(),
            Some(self.a.len() + self.b.len()),
        )
    }
}

impl<'a, T, const N: usize> DoubleEndedIterator for Iter<'a, T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.b.next_back() {
            Some(value) => Some(value),
            None => {
                mem::swap(&mut self.a, &mut self.b);
                self.b.next_back()
            }
        }
    }
}

impl<'a, T, const N: usize> ExactSizeIterator for Iter<'a, T, N> {}

impl<'a, T, const N: usize> IntoIterator for &'a StackDequeue<T, N> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T, N>;
    fn into_iter(self) -> Self::IntoIter {
        let (a, b) = self.as_slices();
        Iter {
            a: a.iter(),
            b: b.iter(),
        }
    }
}

pub struct IterMut<'a, T, const N: usize> {
    a: slice::IterMut<'a, T>,
    b: slice::IterMut<'a, T>,
}

impl<'a, T, const N: usize> Iterator for IterMut<'a, T, N> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.a.next() {
            Some(value) => Some(value),
            None => {
                mem::swap(&mut self.a, &mut self.b);
                self.a.next()
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.a.len() + self.b.len(),
            Some(self.a.len() + self.b.len()),
        )
    }
}

impl<'a, T, const N: usize> DoubleEndedIterator for IterMut<'a, T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.b.next_back() {
            Some(value) => Some(value),
            None => {
                mem::swap(&mut self.a, &mut self.b);
                self.b.next_back()
            }
        }
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut StackDequeue<T, N> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T, N>;
    fn into_iter(self) -> Self::IntoIter {
        let (a, b) = self.as_mut_slices();
        IterMut {
            a: a.iter_mut(),
            b: b.iter_mut(),
        }
    }
}

impl<T, const N: usize> Index<usize> for StackDequeue<T, N> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T, const N: usize> IndexMut<usize> for StackDequeue<T, N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<T: fmt::Debug, const N: usize> fmt::Debug for StackDequeue<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

macro_rules! __impl_slice_eq1 {
    ([$($vars:tt)*] $lhs:ty, $rhs:ty) => {
        impl<T, U, const N: usize, $($vars)*> PartialEq<$rhs> for $lhs
        where
            T: PartialEq<U>,
        {
            fn eq(&self, other: &$rhs) -> bool {
                if self.len() != other.len() {
                    return false;
                }
                let (sa, sb) = self.as_slices();
                let (oa, ob) = other[..].split_at(sa.len());
                sa == oa && sb == ob
            }
        }
    }
}

__impl_slice_eq1! { [const M: usize] StackDequeue<T, N>, [U; M] }
__impl_slice_eq1! { [const M: usize] StackDequeue<T, N>, &[U; M] }
__impl_slice_eq1! { [const M: usize] StackDequeue<T, N>, &mut [U; M] }
__impl_slice_eq1! { [] StackDequeue<T, N>, &[U] }
__impl_slice_eq1! { [] StackDequeue<T, N>, &mut [U] }
