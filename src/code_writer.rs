use super::command_type::CommandType;
use super::parser::VmCommand;
use std::path::PathBuf;
use std::io::{Write};
use std::fs::{File,OpenOptions};

pub fn create_file(output_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    File::create(output_path).expect("[ERROR]failed to create file");
    Ok(())
}
pub fn init(output_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().write(true).append(true).create(false).open(output_path).expect("[ERROR]failed to open file");
    let  mut output = "".to_string();
    //@SP = 256
    let init_stuck_pointer = "@256\nD=A\n@SP\nM=D\n".to_string();
    output = format!("{}{}", output, init_stuck_pointer);
    //call sys.init
    let call_sys_init = write_call("Sys.init", 0, "");
    output = format!("{}{}", output, call_sys_init);
    write!(file, "{}", output)?;

    file.flush()?;
    Ok(())
}

pub fn code_write(vm_file: Vec<VmCommand>, output_path: &PathBuf, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = "".to_string();
    let mut l_id: usize = 0;
    let mut function_name = String::new();
    for vm_command in vm_file.iter() {
        l_id = l_id + 1;
        let id = file_name.to_string() + "$" + &l_id.to_string();
        if vm_command.command_type() == None {
            panic!("[ERROR]failed to parse command to command type.");
        }
        let asm_code: String = match  vm_command.command_type().expect("[ERROR]failed to get command type") {
            CommandType::CArithmetic =>  write_arithmetic(vm_command.arg1(), &id),
            CommandType::CPush => write_push(vm_command.arg1(),vm_command.arg2().parse().expect("[ERROR]arg2 of push command is not available"), file_name),
            CommandType::CPop => write_pop(vm_command.arg1(),vm_command.arg2().parse().expect("[ERROR]arg2 of push command is not available"), file_name),
            CommandType::CLabel => write_label(vm_command.arg1(), &*function_name),
            CommandType::CGoto => write_goto(vm_command.arg1(), &*function_name),
            CommandType::CIf => write_if(vm_command.arg1(), &*function_name),
            CommandType::CFunction => {
                function_name = vm_command.arg1().to_string();
                write_function(vm_command.arg1(), vm_command.arg2().parse().expect("[ERROR]arg2 of function command is not available"))
            },
            CommandType::CReturn => write_return(),
            CommandType::CCall => write_call(vm_command.arg1(), vm_command.arg2().parse().expect("[ERROR]arg2 of call command is not available"), &id),
        };
        output = format!("{}{}", output, asm_code);
    }
    let mut file = OpenOptions::new().write(true).append(true).create(false).open(output_path).expect("[ERROR]failed to open file");
    write!(file, "{}", output)?;
    file.flush()?;
    Ok(())
}

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

fn write_push(segment: &str, index: usize, file_name: &str) -> String {
    let mut code: String = match segment {
        "argument" => "@ARG\nA=M\n".to_string(),
        "local" => "@LCL\nA=M\n".to_string(),
        "static" => "@".to_string() + file_name + "." + &index.to_string() + "\n",
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

fn write_pop(segment: &str, index: usize, file_name: &str) -> String {
    let mut code = "@SP\nA=M-1\nD=M\n".to_string();
    let segment_code = match segment {
        "argument" => "@ARG\nA=M\n".to_string(),
        "local" => "@LCL\nA=M\n".to_string(),
        "static" => "@".to_string() + file_name + "." + &index.to_string() + "\n",
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

fn write_label(label_name: &str, function_name: &str) -> String {
    let mut code = "".to_string();
    code = match function_name {
        "" => code + "(" + label_name + ")",
        _ => code + "(" + function_name + "$" + label_name + ")",
    } + "\n";
    return code
}

fn write_goto(label_name: &str, function_name: &str) -> String {
    let mut code = "".to_string();
    code = match function_name {
        ""  => code + "@" + label_name,
        _ => code + "@" + function_name + "$" + label_name,
    } + "\n"
    + "0;JMP\n";
    return code
}

fn write_if(label_name: &str, function_name: &str) -> String {
    let mut code = "".to_string();
    code = code
        + "@SP\n"
        + "A=M-1\n"
        + "D=M\n"
        + "@SP\n"
        + "M=M-1\n";
    code = match function_name {
            "" => code + "@" + label_name,
            _ => code + "@" + function_name + "$" + label_name,
    } + "\n"
    + "D;JNE\n";
    return code
}

fn write_call(function_name: &str, num_args: usize, id: &str) -> String {
    let mut code = "".to_string();
    //push return-address
    code = code
        + "@" + function_name + "return" + id + "\n"
        + "D=A\n"
        + "@SP\n"
        + "A=M\n"
        + "M=D\n"
        + "@SP\n"
        + "M=M+1\n";
    //push LCL
    code = code
        + "@LCL\n"
        + "D=M\n"
        + "@SP\n"
        + "A=M\n"
        + "M=D\n"
        + "@SP\n"
        + "M=M+1\n";
    //push ARG
    code = code
        + "@ARG\n"
        + "D=M\n"
        + "@SP\n"
        + "A=M\n"
        + "M=D\n"
        + "@SP\n"
        + "M=M+1\n";
    //push THIS
    code = code
        + "@THIS\n"
        + "D=M\n"
        + "@SP\n"
        + "A=M\n"
        + "M=D\n"
        + "@SP\n"
        + "M=M+1\n";
    //push THAT
    code = code
        + "@THAT\n"
        + "D=M\n"
        + "@SP\n"
        + "A=M\n"
        + "M=D\n"
        + "@SP\n"
        + "M=M+1\n";
    //ARG=SP-n-5
    code = code
        + "@SP\n"
        + "D=M\n"
        + "@5\n"
        + "D=D-A\n"
        + "@" + &num_args.to_string() + "\n"
        + "D=D-A\n"
        + "@ARG\n"
        + "M=D\n";
    //LCL=SP
    code = code
        + "@SP\n"
        + "D=M\n"
        + "@LCL\n"
        + "M=D\n";
    //goto f
    code = code
        + "@" + function_name + "\n"
        + "0;JMP\n";
    //label return-address
    code = code
        + "(" + function_name + "return" + id + ")\n";
    return code
}

fn write_return() -> String {
    let mut code = "".to_string();
    //FRAME=LCL
    code = code
        + "@LCL\n"
        + "D=M\n"
        + "@FLAME\n"
        + "M=D\n"
    //RET=*(FLAME -5)
        + "@5\n"
        + "A=D-A\n"
        + "D=M\n"
        + "@RET\n"
        + "M=D\n";
    //*ARG=pop() (ARG 0 = value (SP -1))
    code = code
        + "@SP\n"
        + "AM=M-1\n"
        + "D=M\n"
        + "@ARG\n"
        + "A=M\n"
        + "M=D\n";
    //SP=ARG+1 (@SP-1 is return value)
    code = code
        + "@ARG\n"
        + "D=M+1\n"
        + "@SP\n"
        + "M=D\n";
    //THAT=*(FLAME -1)
    code = code
        + "@FLAME\n"
        + "A=M-1\n"
        + "D=M\n"
        + "@THAT\n"
        + "M=D\n"
    //THIS=*(FLAME -2)
        + "@FLAME\n"
        + "D=M\n"
        + "@2\n"
        + "A=D-A\n"
        + "D=M\n"
        + "@THIS\n"
        + "M=D\n"
    //ARG=*(FLAME -3)
        + "@FLAME\n"
        + "D=M\n"
        + "@3\n"
        + "A=D-A\n"
        + "D=M\n"
        + "@ARG\n"
        + "M=D\n"
    //LCL=*(FLAME -4)
        + "@FLAME\n"
        + "D=M\n"
        + "@4\n"
        + "A=D-A\n"
        + "D=M\n"
        + "@LCL\n"
        + "M=D\n";
    //goto ret
    code = code
        + "@RET\n"
        + "A=M\n"
        + "0;JMP\n";
    return code
}


fn write_function(function_name: &str, n_locals: usize) -> String {
    let mut code = "".to_string();
    code = code
        + "(" + function_name + ")\n";
    //initialaze local varient
    for _x in 0..n_locals {
        code = code
            + &write_push("constant", 0, "");
    }
    return code
}

