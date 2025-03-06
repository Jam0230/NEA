use crate::codegen::codegen::Assembly;
use std::{env, fs, path::PathBuf, process::Command};

impl Assembly {
    pub fn generate_assembly_file(self, file_name: String) {
        // final data for the assembly
        let data_string = format!(
            "section .data\n{}",
            self.data_string
                .split("\n")
                .filter(|&x| !x.is_empty())
                .collect::<Vec<&str>>()
                .join("\n")
        );

        // final code for the assembly
        let text_string = format!(
            "section .text\nmain:\npush rbp\nmov rbp,rsp\nsub rsp,{}\n{}\nadd rsp,{}\npop rbp\nret",
            self.stack_offset,
            self.text_string
                .split("\n")
                .filter(|&x| !x.is_empty())
                .collect::<Vec<&str>>()
                .join("\n"),
            self.stack_offset
        );

        // the final assembly string
        let assembly = format!(
            "global main\nextern printf\nextern puts\n{}\n{}",
            data_string, text_string
        );

        // creating new path for assembly file
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.join(file_name);

        // create new file if one does not exist
        if !path.exists() {
            fs::File::create(path.clone()).expect("Error creating assembly file");
        }

        // write the assembly code to the file
        fs::write(path.clone(), assembly).expect("Error writing to assembly file");

        // compile the assembly using nasm
        self.compile_assembly(path);
    }

    fn compile_assembly(self, path: PathBuf) {
        // get path of assembly file in a string
        let path_as_string = path.into_os_string().into_string().unwrap();

        // get path for the executable file that will be creating when assembling
        let mut executable_path = path_as_string.split('/').collect::<Vec<&str>>();
        executable_path.pop();

        // create and run the command to assemble
        let nasm_cmd = format!(
            "nasm -felf64 {} && gcc -no-pie -o {}/a.out {} && {}/a.out",
            path_as_string,
            executable_path.join("/"),
            path_as_string.replace("asm", "o"),
            executable_path.join("/")
        );

        Command::new("sh")
            .arg("-c")
            .arg(nasm_cmd)
            .spawn()
            .expect("Error assembling assembly code!")
            .wait()
            .expect("Error assembling assembly code!");
    }
}
