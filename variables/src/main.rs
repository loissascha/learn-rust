fn main() {
    const MAX_NUMBER: u32 = 1000;

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    increase_num(&mut x);
    println!("The value of x is: {}", x);

    let y = 5;
    println!("The value of y is: {}", y);

    let sum = sum(&x, &y);
    println!("The sum of x and y is: {}", sum);

    let y = "six";
    println!("The value of y is: {}", y);

    // loops can return values!
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("result of loop is {}", result);

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("while loop {}", number);
        number -= 1;
    }

    // for loop
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("for loop value: {}", element);
    }

    for number in 1..4 {
        println!("{}", number);
    }
}

fn increase_num(num: &mut i32) {
    *num += 1;
}

fn sum(x: &i32, y: &i32) -> i32 {
    *x + *y
}
