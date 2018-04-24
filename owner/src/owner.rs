fn main() {
    // sがスコープに入る
    let s = String::from("hello");

    // sの値が関数にむーぶされる ここではもう有効でない
    takes_ownership(s);

    let x = 5;

    // xも関数にムーブされるうがi32はcopyなので、この後にxを使っても大丈夫
    makes_copy(x);

    println!("x: {}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);

    // ここでsome_stringがスコープをぬけdropが呼ばれる
    // 後ろ盾していたメモリが解放される
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
    // some_integerがスコープを抜けるが,copyのため何も特別なことはない
}
