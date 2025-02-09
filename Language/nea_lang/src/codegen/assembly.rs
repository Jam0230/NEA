use crate::codegen::codegen::Assembly;
use std::{env, fs, path::PathBuf, process::Command};

impl Assembly {
    pub fn generate_assembly_file(self, file_name: String) {
        let data_string = format!(
            "section .data\n{}",
            self.data_string
                .split("\n")
                .filter(|&x| !x.is_empty())
                .collect::<Vec<&str>>()
                .join("\n")
        );
        let text_string = format!(
            "section .text\nmain:\npush rbp\nmov rbp,rsp\nsub rsp,{}\n{}\nadd rsp,8\npop rbp\nret",
            self.stack_offset,
            self.text_string
                .split("\n")
                .filter(|&x| !x.is_empty())
                .collect::<Vec<&str>>()
                .join("\n")
        );

        let assembly = format!(
            "global main\nextern printf\nextern puts\n{}\n{}",
            data_string, text_string
        );

        let current_dir = env::current_dir().unwrap();
        let path = current_dir.join(file_name);
        if !path.exists() {
            fs::File::create(path.clone()).expect("Error creating assembly file");
        }

        fs::write(path.clone(), assembly).expect("Error writing to assembly file");

        self.compile_assembly(path);
    }

    fn compile_assembly(self, path: PathBuf) {
        let path_as_string = path.into_os_string().into_string().unwrap();
        let mut executable_path = path_as_string.split('/').collect::<Vec<&str>>();
        executable_path.pop();

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
