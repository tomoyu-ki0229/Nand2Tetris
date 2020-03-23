use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let no_init_option = args.iter().any(|x| &*x == &*"-noinit".to_string());
    println!("[INFO]start translate vm code to asm code.");
    match vm_translator::vm_translator(&args[1], no_init_option) {
        Ok(()) => println!("[INFO]translate is finished successfully."),
        Err(err) => println!("[ERROR]translate failed.\nerr:{:?}", err),
    };
}

