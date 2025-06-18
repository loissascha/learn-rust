fn main() {
    // INFO:  ---- Ownership rules ----
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    {
        let s = "hello"; // this is on the stack
        let s2 = String::from("world"); // this is on the heap
                                        // stack faster than heap but stack limited (or something like that)
        println!("s is {} {}", s, s2);
    } // scope is over -> s gets dropped

    let x = 5;
    let y = x; // Copy
    println!("x {} y {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1; // Move (not shallow copy)
    println!("{} world", s2); // can't use s1 here because it's been moved

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Copy / clone
    println!("{} {} world", s1, s2); // can still use s1 here

    let s = String::from("hello");
    takes_ownership(s);
    // println!("s {}", s); // can't do that because ownership has been taken

    let x = 33;
    makes_copy(x);
    println!("I still can print out {}", x);

    let s = gives_ownership();
    println!("{}", s);

    let s3 = takes_and_gives_back(s);
    println!("{}", s3);

    let len = calculate_length(&s3);
    println!("{} has length of: {}", s3, len);

    let mut s5 = String::from("change me ");
    change(&mut s5);
    println!("changed {}", s5);
}

fn takes_ownership(some_string: String) {
    println!("I'm the owner now of {}", some_string)
}

// integers are copied
fn makes_copy(some_integer: i32) {
    println!("I copied {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("You are the owner now!");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// use reference to variable to not give ownership
fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn change(s: &mut String) {
    s.push_str("changed!");
}
