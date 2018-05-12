
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
/**
fn generic_largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        // 値が順序付け可能な型のみにしか使用できない
        // 比較を可能にするために 標準ライブラリには型に実装できる
        // std::cmp::PartialOrdトレイトが在る
        if item > largest {
            largest = item;
        }
    }
    largest
}
*/
// ジェネリックを使用した構造体
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let number_list = vec![34, 50, 25, 100, 65];

/**
    let result = generic_largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec!['y', 'm', 'a', 'q'];
    let result = generic_largest(&number_list);
    println!("The largest number is {}", result);
*/
    let both_integer = Point {x:5, y: 10};
    let both_float = Point {x: 1.0, y: 4.0};
    let integer_and_float = Point {x: 5, y: 4.0};

    println!("{}, {}, {}", both_integer.x, both_float.x, integer_and_float.x);

    let p1 = Point {x: 5, y: 10.4};
    let p2 = Point {x: "hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
