struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user = build_user("crab@example.com".to_string(), "Crab".to_string());
    println!("User info: {} {} {} {}", 
             user.active,
             user.username,
             user.email,
             user.sign_in_count);
}

// Function that uses field init shorthand syntax, the parameter
// names have the same name as struct fields, which reduces typing
// inside the function.
fn build_user(email: String, username: String) -> User {
    User {
        active:true,
        username,
        email,
        sign_in_count: 0,
    }
}
