use which::which;

pub fn exists(command: &str) -> bool {
    which(command).is_ok()
}
