enum Actions{
    Echo,
    Error
}

pub fn execute(parse_vec : &Vec<String>) {
    let mut action = Actions::Error;
    if parse_vec[0] == String::from("echo"){
        action = Actions::Echo;
    }

    match action{
        Actions::Echo => {
            let mut output: String = String::new();
            for i in 1..parse_vec.len(){
                output = format!("{} {}", output, parse_vec[i]);
            }
            println!("{}", output);
            
        },
        Actions::Error => {println!("Invalid command")}
    }
}