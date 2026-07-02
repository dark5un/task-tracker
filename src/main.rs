use mini_clap::Command;

fn main() {
    let cmd = Command::new("task-tracker")
        .about("A simple task tracker");

    match cmd.parse(&[]).map(|m| m.command_name) {
        Ok(name) => println!("{name} ready"),
        Err(e) => eprintln!("{e}"),
    }
}