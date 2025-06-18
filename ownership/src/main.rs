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
}
