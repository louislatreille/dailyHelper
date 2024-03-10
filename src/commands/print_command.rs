use crate::commands::Command;

pub struct PrintCommand {
    message: String,
}

impl PrintCommand {
    pub fn new(message: String) -> Self {
        PrintCommand { message }
    }
}

impl Command for PrintCommand {
    fn execute(&self) {
        println!("Print command received: {}", self.message);
    }
}
