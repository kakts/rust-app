fn main() {
    println!("Hello, world!");

    // 型注釈をいれている
    // let v: Vec<i32> = Vec::new()

    // 初期値を与えたので型注釈は必要ない
    let mut v = vec![1, 2, 3];
    v.push(5);
    v.push(6);

    println!("{}, {}", &v[0], &v[1]);

    println!("vector loop");
    for i in &v {
        println!("{}", i);
    }
}
