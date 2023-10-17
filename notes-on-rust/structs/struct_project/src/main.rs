struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user = User {
        active: true,
        username: String::from("Ferris Rustacean"),
        email: String::from("ferris.rustaecean@ocean.com"),
        sign_in_count: 0,
    };

    println!("{}", user.username);
    println!("{}", user.email);
}
