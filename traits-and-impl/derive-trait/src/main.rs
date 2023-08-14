#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

use std::fmt;

impl  fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p1 = Point {x: 5, y: 10};
    let p2 = Point {x: 10, y: 20};

    if p1 == p2 {
        println!("equal");
    }else {
        println!("not equal");
    }

    println!("{}", p1);
    println!("{}", p2);
}