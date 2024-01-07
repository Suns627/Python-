use std::fs;
use std::env;
use std::collections::HashMap;
use std::process::Command;
use std::num::ParseIntError;

fn main() {
    let name: Vec<String> = env::args().collect();
    let content: String = fs::read_to_string(&name[1]).expect("Read file error.Maybe this file is not found.\n");

    let line_contents: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    let mut int_var: HashMap<String,u64> = Default::default();
    let mut string_var: HashMap<String,String> = Default::default();
    for line in line_contents {
        if line[..10].to_string() == "putstring "{
            let var_name: String = line[10..].to_string();
            let value: String;
            match string_var.get(&var_name) {
                Some(s) => value = s.to_string(),
                None => {
                    value = String::from("value not found!");
                }
            }
            print!("{}",value);
        } else if line[..6].to_string() == "print "{
            let print: String = line[6..].to_owned();
            print!("{}",&print);
        } else if line[..8].to_string() == "println "{
            let print: String = line[8..].to_owned();
            println!("{}",&print);
        } else if line[..4].to_string() == "int " {
            Command::new("find_string").arg(&line).arg(":").arg("=").output().unwrap();
            let name = fs::read_to_string("temp.log").expect("read temp file error.Maybe the temp.log is not found.\n");
            Command::new("find_string").arg(&line).arg("=").arg(";").output().unwrap();
            let value = fs::read_to_string("temp.log").expect("read temp file error.Maybe the temp.log is not found.\n");
            let u_value: Result<u64, ParseIntError> = String::from_utf8(value.into()).unwrap().parse();

            if let Ok(value) = u_value {
                int_var.insert(name, value);
            }
        } else if line[..7].to_string() == "string:" {
            Command::new("find_string").arg(&line).arg(":").arg("=").output().unwrap();
            let name = fs::read_to_string("temp.log").expect("read temp file error.Maybe the temp.log is not found.\n");
            Command::new("find_string").arg(&line).arg("=").arg(";").output().unwrap();
            let value = fs::read_to_string("temp.log").expect("read temp file error.Maybe the temp.log is not found.\n");
            string_var.insert(name, value);
        }
    }
}
