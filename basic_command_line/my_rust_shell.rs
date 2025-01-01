use std::io::{self, Write};
use std::process::{self, Command};

const EXIT_SUCCESS: i32 = 0;
// const EXIT_FAILURE: i32 = 1;

const PROMPT: &str = "my_rust_shell> ";

fn builtin_cmd(argv: &Vec<&str>) -> bool {
    match argv[0] {
        "quit" => {
            process::exit(EXIT_SUCCESS);
        }
        _ => {
            return false;
        }
    }
}

fn evaluate(cmd_line: &str) {
    if cmd_line.is_empty() {
        return;
    }

    let argv: Vec<&str> = cmd_line.split_whitespace().collect();
    if !builtin_cmd(&argv) {
        let mut child = Command::new(argv[0])
            .args(&argv[1..])
            .spawn()  // 'output' is blocking as parent waits on child while 'spawn' is not
            .expect("Failed to start child process");  // TODO: breaks program here if program is not found
        let _exit_code = child.wait().expect("Failed while running child process");
    }
}

fn main() {
    let emit_prompt: bool = true;

    loop {
        if emit_prompt {
            print!("{}", PROMPT);
            io::stdout().flush().expect("Failed to flush STDOUT");
        }
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failedto read from STDIN");

        // break if EOF (Ctrl-Z on Windows)
        if input.is_empty() {
            break;
        }

        evaluate(input.trim());
    }

    process::exit(EXIT_SUCCESS);
}
