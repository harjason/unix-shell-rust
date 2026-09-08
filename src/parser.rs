
pub fn parse(cmd: String) -> Vec<String>{
    let mut parsed_vector: Vec<String> = Vec::new(); //contains parsed words
    let mut word: String = String::new();
    for i in 0..cmd.len(){
        let c = cmd.chars().nth(i);
        if c != Some(' '){word.push(c.unwrap_or(' '))}
        else{
            parsed_vector.push(word.clone());
            word.clear();
        }
        if i == cmd.len() -1 {
            parsed_vector.push(word.clone());
            word.clear();
        }
    } 
   parsed_vector
}