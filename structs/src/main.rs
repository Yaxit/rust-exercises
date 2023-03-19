fn main() {
    let usr1 = User {
        active:true,
        username:String::from("Yaxit"),
        email:String::from("yax@at.com"),
        sign_in_count:0
    };

    let mut usr2 = build_user("Trix", "3x@mail.com");

    usr2.email = usr1.email;
    
    let usr3 = User {
        email:String::from("mail@mail.com"),
        ..usr1 // Struct update from usr1, convenient for fast init
        // however it uses =, which means it MOVES the data (aka possibly cannot use usr1 anymore)
    };

    // println!("Here is a User {}", usr1)// User does not implement Display, cannot Print
    println!("Here is a User: {:?}", usr3);// Can use if opeted in with #[derive(Debug)]
    // can also use dbg! macro instead
    dbg!(usr3);


}

#[derive(Debug)]
struct User {
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64
}

struct Color(u8, u8, u8);   // like a struct, but without name fields
struct Point(u8, u8, u8);   // Point is different from Color, aka cannot interchange them in functions etc

fn build_user(username:&str, email:&str) -> User {
    return User {
        active:true,
        username:String::from(username),
        email:String::from(email),
        sign_in_count:1
    };
}

