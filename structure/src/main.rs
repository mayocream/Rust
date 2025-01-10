#[allow(dead_code)]
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = build_user(String::from("someone"), String::from("someone@exmaple.com"));
    println!("{:#?}", user1);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{:#?}", user2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
