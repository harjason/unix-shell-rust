use std::io;
use std::io::Write;

pub mod parser;

fn main() {

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
        let _parsed_vector = parser::parse(command.trim().to_string());
        //.trim() returns a immutable reference &str while .to_stirng allocates a new string on the heap and no ownership is moved
        println!("{:?}" , _parsed_vector)
    }
}
