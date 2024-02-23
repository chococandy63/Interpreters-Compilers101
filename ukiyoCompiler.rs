//defining the Ukiyo module

pub mod Ukiyo {
    pub fn error(line: usize, message: String) {
        eprintln!("[line {}] Error: {}", line, message);
    }
}