mod commands;

const CMDS: &[&str] = &["fastfetch", "neofetch", "gofetch", "hyfetch", "cpufetch"];

fn main() {
    let mut existing: usize = 0;
    let all_count = CMDS.len();
    for s in CMDS {
        if commands::exists(s) {
            println!("{s}");
            existing = existing + 1;
        } else {
            println!("!{s}");
        }
    }
    println!("Existing: {existing} out of {all_count}");
}
