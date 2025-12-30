use crate::iter::MyIterator;

pub struct Filter<I, P> {
    inner: I,
    predicate: P,
}

impl <I, P> Filter<I, P> {
    pub fn new(iter: I, predicate: P) -> Self {
        Self {
            inner: iter,
            predicate,
        }
    }
}

impl<I, P> MyIterator for Filter<I, P>
where
    I: MyIterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.inner.next() {
            if (self.predicate)(&x) {
                return Some(x);
            }
        }

        None
    }
}

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
