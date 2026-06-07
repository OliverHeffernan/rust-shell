use std::io;
use std::process::{Command, exit};
use std::os::unix::process::CommandExt;
use fork::{fork, Fork, waitpid};

/// Parses a command string into a Command struct
fn parse_command(input: &str) -> Command {
    let mut iter = input.split_whitespace();
    let prog = iter.next().unwrap();

    let mut command = Command::new(prog);

    while let Some(arg) = iter.next() {
        command.arg(arg);
    }
    return command;
}

/// Runs a command in a child process,
/// the parent waits for the child to finish
fn run_command(command: &mut Command) {
    match fork() {
        Ok(Fork::Parent(child)) => {
            waitpid(child).expect("Failed to wait for child process");
        }
        Ok(Fork::Child) => {
            let err = Command::exec(command);
            eprintln!("{}", err);
            exit(1);
        }
        Err(e) => {
            eprintln!("Fork failed: {}", e);
        }
    }
}
fn prompt() {
    print!("> ");
    io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");
}
fn main() {
    loop {
        prompt();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == "exit" {
            break;
        }
        let mut command = parse_command(&input);
        run_command(&mut command);
    }
}
