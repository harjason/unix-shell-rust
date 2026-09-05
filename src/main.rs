use std::io;
use std::io::Write;

fn main() {
    println!("welcome to my unix shell");
    let mut command: String = String::new();
    loop{
        command.clear();
        print!("user@computer  ");
        io::stdout().flush();
        let _ = io::stdin().read_line(&mut command);
        if command.trim() == "exit" {break;}
    }
}
