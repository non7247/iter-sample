use crate::iter::MyIterator;
use std::fmt::Debug;

pub struct Map<I, F> {
    inner: I,
    f: F,
}

impl<I, F> Map<I, F> {
    pub fn new(iter: I, f: F) -> Self {
        Self { inner: iter, f }
    }
}

impl<I, F, B> MyIterator for Map<I, F>
where
    I: MyIterator,
    F: FnMut(I::Item) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(&mut self.f)
    }
}

impl<I, F> Debug for Map<I, F>
where
    I: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Map")
            .field("inner", &self.inner)
            .field("f", &"|x| ...")
            .finish()
    }
}
