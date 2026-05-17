mod commands;

const CMDS: &[&str] = &["fastfetch", "neofetch", "gofetch", "hyfetch", "cpufetch"];

fn main() {
    for s in CMDS {
        if commands::exists(s) {
            println!("{s}");
        }
        else {
            println!("!{s}");
        }
    }
}
