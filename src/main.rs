use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path: &str = "C:/Users/Evan Raiford/IdeaProjects/rust_mini_project/src/parser_in.txt";

    let file_string: String = fs:: read_to_string(path).expect("Error in reading the file");

    for line in file_string.lines(){
        println!("{}", parse_instruction(line));

    }
    Ok(())
}

fn parse_instruction(line: &str) -> String{

    let instruction_vec: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();

    if instruction_vec.is_empty() {
        return "BLANK LINE".to_string();
    }

    let mut instruction_binary: String;

    match instruction_vec.as_slice()[0]{

        "add"=> {
            //000000 is arithmetic opcode
            instruction_binary = String::from("000000");

            //read three registers
            instruction_binary.push_str(&*read_register(instruction_vec.as_slice()[1]));
            instruction_binary.push_str(&*read_register(instruction_vec.as_slice()[2]));
            instruction_binary.push_str(&*read_register(instruction_vec.as_slice()[3]));

            //shamt not used
            instruction_binary.push_str(" 00000");

            //32 is the add function
            instruction_binary.push_str(" 100000");


        },
        "sub"=>{
            //000000 is arithmetic opcode
            instruction_binary = String::from("000000");

            //read three registers
            instruction_binary.push_str(&*read_register(instruction_vec.as_slice()[1]));
            instruction_binary.push_str(&*read_register(instruction_vec.as_slice()[2]));
            instruction_binary.push_str(&*read_register(instruction_vec.as_slice()[3]));

            //shamt not used
            instruction_binary.push_str(" 00000");

            //34 is the sub function
            instruction_binary.push_str(" 100010");
        },
        "addi"=> {
            //000000 is arithmetic opcode
            instruction_binary = String::from("000000");

            //read 2 registers
            instruction_binary.push_str(&*read_register(instruction_vec.as_slice()[1]));
            instruction_binary.push_str(&*read_register(instruction_vec.as_slice()[2]));

            //read the 16 bit decimal value and translate it to binary
            let immediate = translate_to_binary(instruction_vec.as_slice()[3]);
            instruction_binary.push_str(&*(" ".to_string() + &*immediate));
        },
        //currently only recognizes comments that start at the beginning of the line.
        //cannot recognize comments that do not have a space after # (eg #comment
        "#"=>instruction_binary = String::from("COMMENT"),
        _=>instruction_binary = String::from("Instruction Not Recognized"),
    };

    return instruction_binary.to_string();
}

fn read_register(listed_register: &str) -> String{


    let cleaned_register : &str = &*listed_register.replace(",", "");

    let mut register_string = "";
    match cleaned_register{
        "$zero"=> register_string = "00000",//0
        "$at"=> register_string = "00001",//1

        "$v0"=> register_string = "00010",//2
        "$v1"=> register_string = "00011",//3

        "$a0"=> register_string = "00100",//4
        "$a1"=> register_string = "00101",//5
        "$a2"=> register_string = "00110",//6
        "$a3"=> register_string = "00111",//7

        "$t0"=>  register_string = "01000",//8
        "$t1"=> register_string = "01001",//9
        "$t2"=> register_string = "01010",//10
        "$t3"=> register_string = "01011",//11
        "$t4"=> register_string = "01100",//12
        "$t5"=> register_string = "01101",//13
        "$t6"=> register_string = "01110",//14
        "$t7"=> register_string = "01111",//15

        "$s0"=>  register_string = "10000",//16
        "$s1"=> register_string = "10001",//17
        "$s2"=> register_string = "10010",//18
        "$s3"=> register_string = "10011",//19
        "$s4"=> register_string = "10100",//20
        "$s5"=> register_string = "10101",//21
        "$s6"=> register_string = "10110",//22
        "$s7"=> register_string = "10111",//23

        "t8"=>  register_string = "11000",//24
        "$t9"=> register_string = "11001",//25

        "$k0"=> register_string = "11010",//26
        "$k1"=> register_string = "11011",//27

        "$gp"=> register_string = "11100",//28
        "$sp"=> register_string = "11101",//29
        "$fp"=> register_string = "11110",//30
        "$ra"=> register_string = "11111",//31

        _=>println!("Destination Register not recognized."),
    };

    return " ".to_string() + &*register_string.to_string();

}

//takes string representation of an int and returns string representation of the binary equivalent.
//need to update it to properly display 16 bit length 2's complement negatives
fn translate_to_binary(given_text: &str) -> String{

    let decimal: i32 = given_text.parse().unwrap();

    let mut binary_representation = format!("{:b}", decimal);

    //denotes if the immediate is too big for MIPS
    if decimal > 32767 || decimal < -32768 {
        println!("Immediate value is too big. Must be within 16-bits: -32768 to 32767.");
    }

    //sign extends to 16 bits for positive ints
    if decimal >= 0 {
        let extend_binary = "0".to_string();
        while binary_representation.len() < 16 {
            binary_representation = extend_binary.to_string() + &*binary_representation.to_string();
        }
    //trims negative ints down to 16 bits
    }else if decimal < 0{
        while binary_representation.len() > 16 {
            binary_representation.remove(0);
        }
    }


    return  binary_representation.to_string();

}
