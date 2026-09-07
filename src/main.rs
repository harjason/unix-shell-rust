use std::io;
use std::io::Write;

//module definitions
pub mod parser;


fn main() {
    parser::check();
    println!("welcome to my unix shell");
    print!("Enter a username: ");
    let _ = io::stdout().flush(); 
    let mut username: String = String::new();
    let _ = io::stdin().read_line(&mut username);

    let pre: String = format!("{}@shell", username.trim_end());

    let mut command: String = String::new();
    
    loop{
        command.clear();
        print!("[{}]$   ",pre);
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut command);
        if command.trim() == "exit" {break;}
    }
}
