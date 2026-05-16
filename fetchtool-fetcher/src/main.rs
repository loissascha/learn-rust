mod commands;

fn main() {
    if commands::exists("fastfetch") {
        println!("fastfetch available!");
    } else {
        println!("fastfetch not available!");
    }
}
