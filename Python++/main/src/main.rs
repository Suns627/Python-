use std::fs;
use std::env;
use std::collections::HashMap;
use std::process::Command;
use std::num::ParseIntError;

fn main() {
    let name: Vec<String> = env::args().collect();
    let content: String = fs::read_to_string(&name[1]).expect("Read file error");

    let line_contents: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    let mut int_var: HashMap<String,u64> = Default::default();
    let mut string_var: HashMap<String,String> = Default::default();
    for line in line_contents {
        if line[..6].to_string() == "print "{
            let print: String = line[6..].to_owned();
            print!("{}",&print);
        }
        if line[..4].to_string() == "int " {
            let name_command = Command::new("find_string").arg(&line).arg(":").arg("=").output().unwrap();
            let name = fs::read_to_string("temp.txt").expect("read temp file error");
            let value_command = Command::new("find_string").arg(&line).arg("=").arg(";").output().unwrap();
            let value = fs::read_to_string("temp.txt").expect("read temp file error");
            let file_contents = fs::read_to_string("temp.txt").expect("read temp file error");  
            let u_value: Result<u64, ParseIntError> = String::from_utf8(file_contents.into()).unwrap().parse();  

            if let Ok(value) = u_value {  
                int_var.insert(name, value);  
            }
        }
        if line[..7].to_string() == "string " {
            let name_command = Command::new("find_string").arg(&line).arg(":").arg("=").output().unwrap();
            let name = fs::read_to_string("temp.txt").expect("read temp file error");
            let value_command = Command::new("find_string").arg(&line).arg("=").arg(";").output().unwrap();
            let value = fs::read_to_string("temp.txt").expect("read temp file error");
            let file_contents = fs::read_to_string("temp.txt").expect("read temp file error");  

            string_var.insert(name, value);
        }
    }
}