struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {

    //explicitly defining an instance
    //is mutable
    let mut user1 = User {
        username: String::from("examplename"),
        email: String::from("example@email.com"),
        active: true,
        sign_in_count: 57
    };

    println!("User info: \nusername: {}\nemail: {}\nactive: {}\nsign in count: {}\n\n", user1.username, user1.email, user1.active, user1.sign_in_count);
    
    //changing a field in a mutable instance
    user1.email = String::from("new@email.com");

    println!("New user info: \nusername: {}\nemail: {}\nactive: {}\nsign in count: {}\n\n ", user1.username, user1.email, user1.active, user1.sign_in_count);

    //using a builder function to create an instance
    let user2 = build_user(String::from("build@email.com"), String::from("builder"));
    println!("User info: \nusername: {}\nemail: {}\nactive: {}\nsign in count: {}\n\n", user2.username, user2.email, user2.active, user2.sign_in_count);


    //creating new instance from existing one the normal way:
    let user3 = User {
        email: user1.email,
        username: String::from("bruhmoment"),
        active: user1.active,
        sign_in_count: user1.sign_in_count
    };

    //creating new instance from existing one using "struct update syntax" if you're only gonna change a handful of the fields:
    let user4 = User {
        email: String::from("bruh@bruh.com"),
        ..user1 // .. syntax specifies that the remaining fields not explicitly set shoud have the same value as the fields in the given instance
    };
    
}


//build and return an instance w/ a function
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}