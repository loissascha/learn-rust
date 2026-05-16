use which::which;

pub fn command_exists(command: &str) -> bool {
    which(command).is_ok()
}
