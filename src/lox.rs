pub struct Lox {
    had_error: bool,
}

trait Report {
    fn error(&mut self, line: usize, message: &String);
    fn report(&mut self, line: usize, location: &String, message: &String);
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }
}

impl Report for Lox {
    fn error(&mut self, line: usize, message: &String) {
        self.report(line, &String::from(message), &message);
    }
    fn report(&mut self, line: usize, location: &String, message: &String) {
        println!("[line {}] Error{}: {}", line, location, message);
        self.had_error = true;
    }
}
