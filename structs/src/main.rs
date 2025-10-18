struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
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

    println!("rect: {:#?}", rect); // <- Debug print structs

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}

// use struct methods instead (look on top)
fn area(rect: &Rectangle) -> u32 {
    // <- & = reference because we don't want to take ownership
    rect.width * rect.height
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
}
