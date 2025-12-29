use std::ops;

pub trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub struct StdIter<T>(pub T);

impl <T: MyIterator> Iterator for StdIter<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
