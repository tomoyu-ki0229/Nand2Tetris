mod command_type;
mod parser;
mod code_writer;

pub fn vm_translator(translate_path: &str, no_init_option: bool) -> Result<(), Box<dyn std::error::Error>> {
    let path: std::path::PathBuf  = {
        let mut path = std::path::PathBuf::new();
        path.push(translate_path);
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
    code_writer::create_file(&output_path)?;
    if !no_init_option { 
        code_writer::init(&output_path)?;
    }
    translate(&path, &output_path)?;
    Ok(())
}

fn translate(path: &std::path::PathBuf, output_path: &std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>>{
    if path.is_file() {
        if path.extension().unwrap() == "vm" {
            let parsed_vm_file = parser::parse(&path);
            let file_name = {
                path.file_stem().expect("[ERROR]failed to get file name").to_str().expect("[ERRORfailed to convert OS_str to &str")
            };
            code_writer::code_write(parsed_vm_file, &output_path, file_name)?;
        }
    } else {
        for child_path in path.read_dir().expect("[ERRORfailed to read path of child dir") {
            if let Ok(child_path) = child_path {
                translate(&child_path.path(), &output_path)?;
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
