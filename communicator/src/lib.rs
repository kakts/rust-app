pub mod client;

pub mod network;




#[cfg(test)]
mod tests {
    // ここでは testsモジュール内になるので、一個階層を上げてclientにアクセスするには
    // ::を先頭につける
    // それか super::をつけるのでも良い
    // useすることでtestsモジュール内にsuper::clientにアクセスできるようにする
    use super::client
    #[test]
    fn it_works() {

        client::connect();
    }
}
