#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username:String)->User{
    User{
        email:String::from(format!("{}@test.com",username)),
        username,
        sign_in_count:0,
        active:true,
    }
}

fn main() {

    let mut user1=build_user(String::from("mi0256"));

    dbg!(&user1);

    user1.email=String::from("test@test.com");

    dbg!(&user1);

}
