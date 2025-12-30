use crate::iter::MyIterator;
use std::mem;

pub struct Skip<I> {
    inner: I,
    remaining: usize,
}

impl<I> Skip<I> {
    pub fn new(iter: I, n: usize) -> Self {
        Self {
            inner: iter,
            remaining: n,
        }
    }
}

impl<I> MyIterator for Skip<I>
where
    I: MyIterator
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let n = mem::take(&mut self.remaining);
        if n == 0 {
            self.inner.next()
        } else {
            self.inner.nth(n)
        }
    }
}
