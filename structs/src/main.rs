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

// struct methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// one struct can (but doesnt need to) have multiple impl blocks (later useful for traits)

// associated functions
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
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

    let rect2 = Rectangle {
        width: 15,
        height: 30,
    };
    let rect3 = Rectangle {
        width: 50,
        height: 70,
    };

    println!("rect can hold rect2 {}", rect.can_hold(&rect2));
    println!("rect can hold rect3 {}", rect.can_hold(&rect3));

    let rect4 = Rectangle::square(50);

    println!("square rect: {:#?}", rect4);
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
