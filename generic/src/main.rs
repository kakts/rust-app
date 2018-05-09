
fn largest(list: &[i32]) -> i32 {
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
}
