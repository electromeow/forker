use std::env::args_os;
use std::process::Command;

fn main() {
    let mut command = args_os();
    command.next();
    let mut child = Command::new(match command.next() {
        Some(a) => a,
        None => {
            eprintln!("usage: forker command_to_fork");
            std::process::exit(1);
        }
    });
    loop {
        let arg = match command.next() {
            Some(a) => a,
            None => break,
        };
        child.arg(arg);
    }

    let pid = child.spawn().unwrap().id();
    eprintln!("Child process started with PID {}", pid);
}
