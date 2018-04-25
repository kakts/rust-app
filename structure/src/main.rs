struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    println!("Hello, world!");

    // user1のフィールドを変更にするためには インスタンス全体が可変である必要がある
    // 一部のフィールドのみ可変はできない
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    println!("email: {}", user1.email);

    user1.email = String::from("hello@example.com");

    let mut user2 = build_user(String::from("test1@test.com"), String::from("test1"));
    println!("{}:{}", user2.email, user2.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
