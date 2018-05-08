use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // matchでエラーの場合 エラーを呼び出し元に返す
    match.f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}


fn read_username_from_file_imp() -> Result<String, io::Error> {
    // ?を使った場合OkならOkの中身が式から帰ってきてプログラム継続
    // エラーの場合returnを使ったかのように関数全体からErrの中身が帰ってきて、
    // 呼び出し元のコードに移譲され、そこで処理がおわる

    // ? と matchのち外
    // ?: 標準ライブラリのFromトレイとで定義され、エラー型を別のものに変換するfrom関数を通る
    // いろんな関数の失敗を一つのエラー型で表現して返すときに有用
    // ?はResultを返す関数の中でしか利用できない
    // match:
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;


    // ?を使ったコードを連結することでさらに簡略化できる
    Ok(s)
}

fn read_username_from_file_imp2() -> Result<String, io::Error> {



    // ?を使ったコードを連結することでさらに簡略化できる
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
