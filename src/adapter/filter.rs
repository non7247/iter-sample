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
