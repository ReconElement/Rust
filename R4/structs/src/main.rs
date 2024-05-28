use std::io;
fn main() {
    let user1: User = User{
        active: true,
        username: String::from("Omkar Panda"),
        email: String::from("omkarpanda895@gmail.com"),
        sign_in_count: 20
    };
    let mut user2 = User{
        active: true,
        username: String::from("Ashutosh Padhi"),
        email: String::from("ashutosh.poggy@poggys.com"),
        sign_in_count: 11
    };
    let user4 = User{
        username: String::from("Omkar Panda"),
        ..user1
    };
    user2.email = String::from("ashutosh.padhi@poggys.com");
    let mut username = String::new();
    let mut email= String::new();
    println!("Enter username and password: ");
    io::stdin().read_line(&mut username).expect("Could not read the username");
    io::stdin().read_line(&mut email).expect("Could not read the email");
    let user3 = build_user(email, username);
    let black = color(0,0,0);
    let point = point(0,0,0);

}
struct color(i32, i32, i32);
struct point(i32, i32, i32);
fn build_user(email: String, username: String)->User{
    User{
        active: true,
        username, //means the same thing as username: username
        email, //means the same thing as email: email
        sign_in_count: 1
    }
}
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
