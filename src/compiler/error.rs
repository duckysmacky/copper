use std::process::Output;

pub struct CompilerError {
    exit_code: i32,
    output: Output,
}

impl CompilerError {
    pub fn new(output: Output) -> Self {
        Self { exit_code: output.status.code().unwrap_or(1), output }
    }

    pub fn print_output(&self, indent: usize) {
        let indent_str = " ".repeat(indent);
        
        if !self.output.stderr.is_empty() {
            let stderr = String::from_utf8_lossy(&self.output.stderr);
            for line in stderr.lines() {
                eprintln!("{}{}", indent_str, line);
            }
        }
        
        if !self.output.stdout.is_empty() {
            let stdout = String::from_utf8_lossy(&self.output.stdout);
            for line in stdout.lines() {
                println!("{}{}", indent_str, line);
            }
        }
    }

    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }
}
