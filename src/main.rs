use std::env;
use std::fs;

struct Variable {
    name: String,
    value: String,
}

static mut VARIABLES: Vec<Variable> = vec![];

fn main() {
    unsafe {
        let args: Vec<String> = env::args().collect();
        let main_file_path = args[1].clone();

        let mut line_number = 0;
        loop {
            let main_data = fs::read_to_string(&main_file_path).expect("Unable to read file");
            let main_data = main_data.replace("\r", "");
            let lines: Vec<&str> = main_data.split("\n").collect();

            let line = lines[line_number].to_string();
            let words: Vec<&str> = line.split(" ").collect();

            let mut i = 0;
            while i < words.len() - 1{
                match words[i] {
                    "let" => {
                        i += 1;
                        let var = Variable {
                            name: words[i].trim().to_owned(),
                            value: words[i+1].trim().to_string(),
                        };
                        VARIABLES.push(var);
                    }

                    "print" => {
                         
                    }

                    _ => {}
                }
                i += 1
            }

            line_number += 1;
        }
    }
}

fn find_var(name: String) -> Variable {
    for varia
}
