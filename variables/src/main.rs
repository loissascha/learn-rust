fn main() {
    const MAX_NUMBER: u32 = 1000;

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    increase_num(&mut x);
    println!("The value of x is: {}", x);

    let y = 5;
    println!("The value of y is: {}", y);
    let y = "six";
    println!("The value of y is: {}", y);
}

fn increase_num(num: &mut i32) {
    *num += 1;
}
