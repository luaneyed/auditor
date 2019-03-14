mod module;
mod traverser;

use std::env;
use std::io;
use std::str::FromStr;
use std::path::{Path, PathBuf};
use serde_json::{self};
use std::process::Command;
use std::str;
use serde_json::value::{Value, Value::Object};
use module::{AuditResult, Severity};
use traverser::Traverser;

const PARSING_ERROR_MSG: &str = "An error occured  while parsing the result of 'npm audit'";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let root = args.get(1).expect("Please enter the root directory as an argument.");
    let root_path = Path::new(root).to_owned();

    // dbg!(root_path);

    let traverser = Traverser::new(root_path);

    let mut result = AuditResult::new();;
    for dir in traverser {
        result.merge(get_audit(&dir));
    }

    println!("{}", result);

    Ok(())
}

fn get_audit(dir: &PathBuf)-> AuditResult {
    println!("Auditing {}", dir.to_str().unwrap());

    let output = Command::new("npm")
        .arg("audit")
        .arg("--json")
        .current_dir(dir)
        .output()
        .expect("failed to execute process")
        .stdout;

    let output = str::from_utf8(&output).expect(PARSING_ERROR_MSG);

    let json: Value = serde_json::from_str(output).expect(PARSING_ERROR_MSG);

    if let Object(map) = json {
        let mut result = AuditResult::new();

        if let Some(advisories) = map.get("advisories") {
            if let Object(map) = advisories {
                for (_, advisory) in map.iter() {
                    if let Object(map) = advisory {
                        let module_name = map.get("module_name").expect(PARSING_ERROR_MSG).as_str().expect(PARSING_ERROR_MSG);
                        let severity = map.get("severity").expect(PARSING_ERROR_MSG).as_str().expect(PARSING_ERROR_MSG);
                        let severity = Severity::from_str(severity).unwrap();
                        result.add_advisory(module_name, severity);
                    } else {
                        panic!(PARSING_ERROR_MSG);
                    }
                }
            } else {
                panic!(PARSING_ERROR_MSG);
            }
        } else {
            println!("{} directory has no advisory", dir.to_str().unwrap());
        }
        return result
    }

    panic!(PARSING_ERROR_MSG);
}
