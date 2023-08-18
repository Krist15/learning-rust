#[derive(Debug)]
struct Counter {
    length: usize,
    count: usize,
    
}

impl Counter {
    fn new(length: usize) -> Counter {
        Counter { length, count: 0 }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        
        self.count += 1;
        
        if self.count <= self.length {
            Some(self.count)
        }else {
            None
        }
    }
}

fn main() {
    for number in Counter::new(10) {
        println!("{}", number);
    }

    let sum_until_10: usize = Counter::new(10).sum();
    assert_eq!(sum_until_10, 55);

    let power_of_2: Vec<usize> = Counter::new(8).map(|n| 2usize.pow(n as u32)).collect();
    assert_eq!(power_of_2, [2, 4, 8, 16, 32, 64, 128, 256]);
}
