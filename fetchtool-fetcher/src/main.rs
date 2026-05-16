use which::which;

fn main() {
    if which("fastfetch").is_ok() {
        println!("fastfetch available!");
    } else {
        println!("fastfetch not available!");
    }
}
