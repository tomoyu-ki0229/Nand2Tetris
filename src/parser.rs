//use
use super::command_type::CommandType;
use std::fs::File;
use std::io::{BufReader,BufRead};

//struct of vm command that is one line in .vm file
pub struct VmCommand {
    vm_commands: Vec<String>,
}

impl VmCommand {

    pub fn new(vm_commands: Vec<String>) -> Self {
        return VmCommand {
            vm_commands: vm_commands
        }
    }

    pub fn command_type(&self) -> Option<CommandType> {
        let _arithmetic_command = ["add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not"];
        let _push_command = ["push"];
        let _pop_command = ["pop"];
        let _label_command = ["label"];
        let _goto_command = ["goto"];
        let _if_command = ["if-goto"];
        let _function_command = ["function"];
        let _call_command = ["call"];
        let _return_command = ["return"];
        let vm_command = &&self.vm_commands[0];

        if _arithmetic_command.iter().any(|command| &command == vm_command) {
            return Some(CommandType::CArithmetic)
        }
        if _push_command.iter().any(|command| &command == vm_command) {
            return Some(CommandType::CPush)
        }
        if _pop_command.iter().any(|command| &command == vm_command) {
            return Some(CommandType::CPop)
        }
        if _label_command.iter().any(|command| &command == vm_command) {
            return Some(CommandType::CLabel)
        }
        if _goto_command.iter().any(|command| &command == vm_command) {
            return Some(CommandType::CGoto)
        }
        if _if_command.iter().any(|command| &command == vm_command) {
            return Some(CommandType::CIf)
        }
        if _function_command.iter().any(|command| &command == vm_command) {
            return Some(CommandType::CFunction)
        }
        if _call_command.iter().any(|command| &command == vm_command) {
            return Some(CommandType::CCall)
        }
        if _return_command.iter().any(|command| &command == vm_command) {
            return Some(CommandType::CReturn)
        }
        return None
    }

    //return arg1
    //If this vm command is CArithmetic, this function returns Arithmetic command.
    pub fn arg1(&self) -> &String {
        if self.command_type().unwrap() == CommandType::CArithmetic {
            return &self.vm_commands[0]
        } else {
            return &self.vm_commands[1]
        }
    }

    pub fn arg2(&self) -> &String {
        return &self.vm_commands[2]
    }
}

// parse vm commands of one file
pub fn parse(path: &std::path::PathBuf) -> Vec<VmCommand> {
    let mut vm_file:  Vec<VmCommand> = Vec::new();
    for line_result in BufReader::new(File::open(path).expect(&format!("[ERROR]failed to read line when I parse vm file. path:{:?}", path))).lines() {
        let line = line_result.expect("[ERROR]failed to get line of vm file.");
        let vm_command = if line.contains("//") {
            line.split("//").nth(0).expect("[ERROR]failed to purge comment block").to_string() //purge comment block
        } else {
            line
        };
        if &vm_command != "" {
            vm_file.push(VmCommand::new(vm_command.split_whitespace().map(|x| x.to_string()).collect()));
        }
    }
    return vm_file
}
