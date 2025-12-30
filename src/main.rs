use iter_sample::iter::MyIterator;

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

fn main() {
    let n = RangeUsize::new(0, 5).count();
    println!("[count] 0..5 => {}", n);

    print!("[for_each] 0..5 =>");
    RangeUsize::new(0, 5).for_each(|x| print!(" {}", x));
    println!();

    let last = RangeUsize::new(10, 15).last();
    println!("[last] 10..15 => {:?}", last);

    let sum = RangeUsize::new(1, 6).fold(0usize, |acc, x| acc + x);
    println!("[fold] sum 1..6 => {}", sum);

    let sum = RangeUsize::new(1, 6).sum();
    println!("[sum] sum 1..6 => {}", sum);

    let v = RangeUsize::new(3, 8).collect_vec();
    println!("[collect_vec] 3..8 => {:?}", v);

    let mut it = RangeUsize::new(100, 110);
    let third = it.nth(3);
    println!("[nth] 100..110 nth(3) => {:?}", third);
    let next = it.next();
    println!("[nth] then next() => {:?}", next);

    let mut it = RangeUsize::new(0, 20);
    let found = it.find(|x| *x % 7 == 0 && *x != 0);
    println!("[find] first nonzero multiple of 7 in 0..20 => {:?}", found);
    let after = it.next();
    println!("[find] then next() => {:?}", after);

    print!("[filter] even in 0..10 =>");
    RangeUsize::new(0, 10)
        .filter(|x| *x % 2 == 0)
        .for_each(|x| print!(" {}", x));
    println!();

    let n = RangeUsize::new(0, 100).filter(|x| *x % 3 == 0).count();
    println!("[filter+count] multiples of 3 in 0..100 => {}", n);

    let last_even = RangeUsize::new(0, 10).filter(|x| *x % 2 == 0).last();
    println!("[filter+last] last even in 0..10 => {:?}", last_even);

    let sum_even = RangeUsize::new(1, 11).filter(|x| *x % 2 == 0).sum();
    println!("[filter+fold] sum of evens in 1..11 => {}", sum_even);

    let v = RangeUsize::new(0, 20).filter(|x| *x % 7 == 0).collect_vec();
    println!("[filter+collect_vec] multiples of 7 in 0..20 => {:?}", v);

    let mut it = RangeUsize::new(0, 20).filter(|x| *x % 5 == 0);
    let third = it.nth(3);
    println!("[filter+nth] (0..20).filter(%5==0).nth(3) => {:?}", third);
    let next = it.next();
    println!("[filter+nth] then next() => {:?}", next);
}
