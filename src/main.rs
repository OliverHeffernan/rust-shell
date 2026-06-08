use std::io;
use std::process::{Command, exit};
use std::os::unix::process::CommandExt;
use std::str::SplitWhitespace;

use fork::{fork, Fork, waitpid};

fn parse_quotes(t: &str, input: &mut SplitWhitespace) -> String {
    let mut arg = t[1..].to_string();
    loop {
        arg.push_str(" ");
        if let Some(t) = input.next() {
            if t.ends_with("\"") {
                arg.push_str(&t[..t.len() - 1]);
                break;
            } else {
                arg.push_str(t);
                arg.push_str(" ");
            }
        } else {
            break;
        }
    }
    arg
}

fn parse_arg(input: &mut SplitWhitespace) -> Option<String> {
    let mut arg = String::new();
    loop {
        if let Some(t) = input.next() {
            if t.ends_with("\\") {
                arg.push_str(&t[..t.len() - 1]);
                arg.push_str(" ");
                continue;
            } else if t.starts_with("\"") {
                arg.push_str(&parse_quotes(t, input));
                break;
            }
            return Some(arg + t);
        }
        break;
    }
    return if arg.is_empty() { None } else { Some(arg) }
}

fn parse_args(input: &mut SplitWhitespace) -> Vec<String> {
    let mut args = Vec::new();
    while let Some(arg) = parse_arg(input) {
        args.push(arg);
    }
    args
}

/// Parses a command string into a Command struct
fn parse_command(input: &str) -> Option<Command> {
    let mut char_iter = input.split_whitespace().into_iter();
    let args = parse_args(&mut char_iter);

    let mut iter = args.iter();

    iter.next().map(|prog| {
        let mut command = Command::new(prog);
        while let Some(arg) = iter.next() {
            command.arg(arg);
        }
        command
    })
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

/// Prompts the user for input
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
        if let Some(ref mut cmd) = command {
            run_command(cmd);
        } else {
            continue;
        }
    }
}
