use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path: &str = "C:/Users/Evan Raiford/IdeaProjects/rust_mini_project/src/parser_in.txt";

    let file_string: String = fs:: read_to_string(path).expect("Error in reading the file");

    for line in file_string.lines(){
        parse_instruction(line);

    }
    Ok(())
}

fn parse_instruction(line: &str) -> String{

    let instruction_vec: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();

    if instruction_vec.is_empty() {
        println!("Instruction is empty");
        return "Instruction is empty".to_string();
    }
    match instruction_vec.as_slice()[0]{
        "add"=>println!("Add instruction recognized."),
        "sub"=>println!("Sub instruction recognized."),
         _=>println!("Instruction not recognized!"),
    };

    return instruction_vec.as_slice()[0].to_string();
}

fn read_register(listed_register: &str) -> String{
    let mut register_string = "";
    match listed_register{
        "$zero"=> register_string = "00000",
        "v0"=> register_string = "00001",
        "v1"=> register_string = "00010",
        "a0"=> register_string = "00011",

        
        _=>println!("Destination Register not recognized."),
    };
    return register_string.to_string();

}

