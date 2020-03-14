mod command_type;
mod parser;
mod code_writer;

pub fn vm_translator(command_line_arg: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path: std::path::PathBuf  = {
        let mut path = std::path::PathBuf::new();
        path.push(command_line_arg);
        path
    };
    let output_path = {
        let mut output_path = path.clone();
        if path.is_dir() {
            let dir_name = path.file_name().expect("[ERROR]set get name of dir name failed.");
            output_path.push(dir_name);
        }
        output_path.set_extension("asm");
        output_path
    };
    let f_num: usize = 0;
    translate(&path, &output_path, f_num)?;
    Ok(())
}

fn translate(path: &std::path::PathBuf, output_path: &std::path::PathBuf, mut f_num: usize) -> Result<(), Box<dyn std::error::Error>>{
    if path.is_file() {
        if path.extension().unwrap() == "vm" {
            let _parsed_vm_file = parser::parse(&path);
            f_num = f_num + 1;
            code_writer::code_write(_parsed_vm_file, &output_path, &(f_num.to_string()+ "F"))?;
        }
    } else {
        for child_path in path.read_dir().expect("failed to read path of child dir") {
            if let Ok(child_path) = child_path {
                translate(&child_path.path(), &output_path, f_num)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn parser_parse_test() {
        assert_eq!(2 + 2, 4);
    }
}
