struct User{

    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main(){
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("test@gmail.com");

    println!("{}", user1.email);

    let user2 = build_user(String::from("test2@gmail.com"), String::from("control"));

    println!("{}", user2.email);
}

fn build_user(username: String, email: String) -> User{
    User {
        username,
        email,
        sign_in_count:1, 
        active: true,
    }
}