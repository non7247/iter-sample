trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: usize,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl MyIterator for Counter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.count;
        self.count += 1;
        Some(result)
    }
}

struct RangeUsize {
    start: usize,
    end: usize,
}

impl RangeUsize {
    fn new(start: usize, end: usize) -> RangeUsize {
        RangeUsize { start, end }
    }
}

impl MyIterator for RangeUsize {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let result = self.start;
            self.start += 1;
            Some(result)
        } else {
            None
        }
    }
}

struct StdIter<T>(T);

impl <T: MyIterator> Iterator for StdIter<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

fn main() {
    let mut range = RangeUsize::new(1, 4);
    while let Some(x) = range.next() {
        println!("Raw Range: {}", x);
    }

    let range_std = StdIter(RangeUsize::new(1, 4));
    for x in range_std {
        println!("StdIter: {}", x);
    }
}
