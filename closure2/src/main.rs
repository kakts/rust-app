fn main() {
    let x = 4;

    // equal_to_xの引数でないのに,equal_to_xが定義されているのと同じスコープで定義される
    // x変数をequal_to_xクロージャは使用できる。 
    // これが関数の場合はコンパイルできない
    // これをキャプチャという
    let equal_to_x = |z| z == x;

    let y = 4;
    assert!(equal_to_x(y));
}
