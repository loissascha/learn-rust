use crate::commands::command_exists;

mod commands;

fn main() {
    if command_exists("fastfetch") {
        println!("fastfetch available!");
    } else {
        println!("fastfetch not available!");
    }
}
