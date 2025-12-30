use std::ops;
use crate::adapter::filter::Filter;
use crate::adapter::map::Map;
use crate::adapter::skip::Skip;
use crate::adapter::take::Take;

pub trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;

    fn for_each<F>(mut self, mut f: F)
    where
        F: FnMut(Self::Item),
        Self: Sized,
    {
        while let Some(x) = self.next() {
            f(x);
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        for _ in 0..n {
            self.next()?;
        }
        self.next()
    }

    fn collect_vec(self) -> Vec<Self::Item> 
    where
        Self: Sized,
    {
        let mut vec = Vec::new();
        self.for_each(|x| vec.push(x));
        vec
    }

    fn find<P>(&mut self, mut predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        while let Some(x) = self.next() {
            if predicate(&x) {
                return Some(x);
            }
        }
        None
    }

    fn fold<B, F>(mut self, mut acc: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
        Self: Sized,
    {
        while let Some(x) = self.next() {
            acc = f(acc, x);
        }
        acc
    }

    fn sum(self) -> Self::Item
    where
        Self::Item: ops::Add<Output = Self::Item> + Default,
        Self: Sized,
    {
        self.fold(Self::Item::default(), |acc, x| acc + x)
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.fold(0, |acc, _| acc + 1)
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.fold(None, |_, x| Some(x))
    }

    fn filter<P>(self, predicate: P) -> Filter<Self, P>
    where
        P: FnMut(&Self::Item) -> bool,
        Self: Sized,
    {
        Filter::new(self, predicate)
    }

    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        F: FnMut(Self::Item) -> B,
        Self: Sized,
    {
        Map::new(self, f)
    }

    fn skip(self, n: usize) -> Skip<Self>
    where
        Self: Sized,
    {
        Skip::new(self, n)
    }

    fn take(self, n: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take::new(self, n)
    }
}

pub struct StdIter<T>(pub T);

impl <T: MyIterator> Iterator for StdIter<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
