use std::io::Take;
use std::ops::Add;

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

impl Point<u32> {
    fn sum(&self) -> u32 {
        &self.x + &self.y
    }
}

struct Point2<X, Y>{
    x : X,
    y : Y
}

impl <X, Y> Point2<X, Y>{
    fn mixup(self, other : Point<X>) -> Point2<X, Y>{
        Point2 {
            x: other.x,
            y : self.y
        }
    }
}

pub fn test_generic_struct() {
    let point_1 = Point {
        x: 5,
        y: 3,
    };

    let point_str = Point {
        x: "1",
        y: "2",
    };

    let point_mix = Point2 {
        x: 1,
        y: "2",
    };

    let point_mix = point_mix.mixup(point_1);
    println!("{}", point_str.get_x());
    //println!("{}", point_1.sum());
    println!("{}", point_mix.x);
}


pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for i in list {
        if i > largest {
            largest = i;
        }
    }
     largest
}

pub fn test_largest_generic() {
    let vec = vec!(1, 2, 3, 4);
    let vec2 = vec!("bd", "bb", "bc", "br");
    let arr = [1.4, 1.4, 1.4, 1.4];
    println!("{}", largest(&vec));
    println!("{}", largest(&vec2));
    println!("{}", largest(&arr));
}