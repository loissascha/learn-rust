struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut user1 = User {
        email: String::from("sascha@loishandl.at"),
        username: String::from("Sascha"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("Vally");

    let user2 = build_user(
        String::from("sascha.loishandl@gmail.com"),
        String::from("saschalois"),
    );

    let user3 = User {
        email: String::from("someone@somehwere.com"),
        username: String::from("someone"),
        ..user2
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", area(rect));
}

fn area(dimensions: Rectangle) -> u32 {
    dimensions.width * dimensions.height
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
}
