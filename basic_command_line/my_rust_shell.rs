use std::io::{self, Write};

const EXIT_SUCCESS: i32 = 0;
// const EXIT_FAILURE: i32 = 1;

const PROMPT: &str = "my_rust_shell> ";

fn builtin_cmd(argv: &Vec<&str>) -> bool {
    match argv[0] {
        "quit" => {
            std::process::exit(EXIT_SUCCESS);
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
        // TODO: handle the command by forking a child process that handles it
        println!("NOT a built-in command");
    }
}

fn main() {
    let emit_prompt: bool = true;

    loop {
        if emit_prompt {
            print!("{}", PROMPT);
            io::stdout().flush().unwrap();  // unwrapping means the program will panic if the flush failed
        }
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // break if EOF (Ctrl-Z on Windows)
        if input.is_empty() {
            break;
        }

        evaluate(input.trim());
    }

    std::process::exit(EXIT_SUCCESS);
}
