fn main() {
    let x = vec![1, 2, 3];

    // moveキーワードをつけることで、このタイミングでxの所有権がクロージャに渡る
    let equal_to_x = move |z| z == x;

    // すでにxの所有権は渡っているのでここでエラーになる
    println!("Can't use x here: {:?}", x);
    let y = 4;
    assert!(equal_to_x(y));
}
