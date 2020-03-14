use super::command_type::CommandType;
use super::parser::VmCommand;
use std::path::PathBuf;
use std::io::{Write};
use std::fs::File;

pub fn code_write(vm_file: Vec<VmCommand>, output_path: &PathBuf, f_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = "".to_string();
    let mut l_id: usize = 0;
    for vm_command in vm_file.iter() {
        l_id = l_id + 1;
        let id = f_id.to_string() + &l_id.to_string();
        if vm_command.command_type() != None {
            let asm_code: String = match  vm_command.command_type().expect("\n\n[ERROR]failed to get command type.\n\n") {
                CommandType::CArithmetic =>  write_arithmetic(vm_command.arg1(), &id),
                CommandType::CPush => write_push(vm_command.arg1(),vm_command.arg2().parse().unwrap(), f_id),
                CommandType::CPop => write_pop(vm_command.arg1(),vm_command.arg2().parse().unwrap(), f_id),
                _ => "".to_string(), //do nothing
            };
            output = format!("{}{}", output, asm_code);
        }
    }
    let mut file = File::create(output_path).unwrap();
    write!(file, "{}", output)?;
    file.flush()?;
    Ok(())
}

//let _arithmetic_command = ["add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not"];
fn write_arithmetic(command: &str, id: &str) -> String {
    let code = match command {
        "add" => "@SP\nA=M-1\nD=M\nA=A-1\nM=D+M\n@SP\nM=M-1\n".to_string(),
        "sub" => "@SP\nA=M-1\nD=M\nA=A-1\nM=M-D\n@SP\nM=M-1\n".to_string(),
        "neg" => "@SP\nA=M-1\nM=-M\n".to_string(),
        //true = -1, false = 0
        "eq" => format!("@SP\nA=M-1\nD=M\nA=A-1\nMD=M-D\n@XGTY{}\nD;JEQ\n@SP\nA=M-1\nA=A-1\nM=0\n@END{}\n0;JMP\n(XGTY{})\n@SP\nA=M-1\nA=A-1\nM=-1\n(END{})\n@SP\nM=M-1\n", id, id, id, id).clone(),
        //true = -1, false = 0
        "gt" => format!("@SP\nA=M-1\nD=M\nA=A-1\nMD=M-D\n@XGTY{}\nD;JGT\n@SP\nA=M-1\nA=A-1\nM=0\n@END{}\n0;JMP\n(XGTY{})\n@SP\nA=M-1\nA=A-1\nM=-1\n(END{})\n@SP\nM=M-1\n", id, id, id, id).clone(),
        //true = -1, false = 0
        "lt" => format!("@SP\nA=M-1\nD=M\nA=A-1\nMD=M-D\n@XGTY{}\nD;JLT\n@SP\nA=M-1\nA=A-1\nM=0\n@END{}\n0;JMP\n(XGTY{})\n@SP\nA=M-1\nA=A-1\nM=-1\n(END{})\n@SP\nM=M-1\n", id, id, id, id).clone(),
        "and" => "@SP\nA=M-1\nD=M\nA=A-1\nM=M&D\n@SP\nM=M-1\n".to_string(),
        "or" => "@SP\nA=M-1\nD=M\nA=A-1\nM=M|D\n@SP\nM=M-1\n".to_string(),
        "not"=> "@SP\nA=M-1\nM=!M\n".to_string(),
        _ => "".to_string(),
    };
    return code
}

fn write_push(segment: &str, index: usize, id: &str) -> String {
    let mut code: String = match segment {
        "argument" => "@ARG\nA=M\n".to_string(),
        "local" => "@LCL\nA=M\n".to_string(),
        "static" => "@".to_string() + id + &index.to_string() + "\n",
        "constant" => "@".to_string() + &index.to_string() + "\n",
        "this" => "@THIS\nA=M\n".to_string(),
        "that" => "@THAT\nA=M\n".to_string(),
        "pointer" => "@R3\n".to_string(),
        "temp" => "@R5\n".to_string(),
        _ => "\n".to_string(),
    };
    if segment != "static" && segment != "constant" {
        for _x in 0..index {
            code = code + "A=A+1\n";
        }
    }
    if segment == "constant" {
        code = code + "D=A\n";
    } else {
        code = code + "D=M\n";
    }
    code = code + "@SP\nA=M\nM=D\n@SP\nM=M+1\n";
    return code
}

fn write_pop(segment: &str, index: usize, id: &str) -> String {
    let mut code = "@SP\nA=M-1\nD=M\nM=0\n".to_string();
    let segment_code = match segment {
        "argument" => "@ARG\nA=M\n".to_string(),
        "local" => "@LCL\nA=M\n".to_string(),
        "static" => "@".to_string() + id + &index.to_string() + "\n",
        "constant" => "\n".to_string(),
        "this" => "@THIS\nA=M\n".to_string(),
        "that" => "@THAT\nA=M\n".to_string(),
        "pointer" => "@R3\n".to_string(),
        "temp" => "@R5\n".to_string(),
        _ => "\n".to_string(),
    };
    code = code + &segment_code;
    if segment != "static" {
        for _x in 0..index {
            code = code + "A=A+1\n";
        }
    }
    code = code + "M=D\n@SP\nM=M-1\n";
    if segment == "constant" {
        return "".to_string()
    } else {
        return code
    }
}

