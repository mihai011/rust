struct User{

    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle{
    height: u32,
    width: u32
}

impl Rectangle{
    fn area(&self)->u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.width > other.height
    }

    fn square(size: u32) -> Rectangle{
        Rectangle { height: size, width: size }
    }
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

    let user2=User{
        username: String::from("trusted username"),
        //email: String::from("locale_email"),
        ..user1
    };

    println!("{}",user2.email);

    let rect1 = Rectangle{height:30,width:50};

    let area = area(&rect1);

    println!("{}", area);
    println!("{:#?}", rect1);
    println!("{}",rect1.area());

    let rect2 = Rectangle{height:30,width:50};
    println!("{}", rect1.can_hold(&rect2));
}   

fn area( rect: &Rectangle)-> u32{
    rect.height* rect.width
}

fn build_user(username: String, email: String) -> User{
    User {
        username,
        email,
        sign_in_count:1, 
        active: true,
    }
}