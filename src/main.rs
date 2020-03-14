use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("[INFO]start translate vm code to asm code.");
    match vm_translator::vm_translator(&args[1]) {
        Ok(()) => println!("[INFO]translate is successful."),
        Err(err) => println!("[ERROR]translate failed.\nvale{:?}", err),
    };
}

