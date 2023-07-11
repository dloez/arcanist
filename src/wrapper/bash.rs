use duct::cmd;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use regex::Regex;

const REGEX_FUNCTION_PATTERN: &str = r"[ \n\t]{{function_name}}[ \n\t]*\(.*\)[ \n\t]*\{";

pub fn does_function_exists(spec_file_path: &Path, function_name: &str) -> bool {
    let file_content = fs::read_to_string(spec_file_path)
        .unwrap_or_else(|_| panic!("Failed to read {:?}", spec_file_path));

    let regex_pattern = &REGEX_FUNCTION_PATTERN.replace("{{function_name}}", function_name);
    let regex = Regex::new(regex_pattern).expect("Failed to define regex");
    regex.is_match(&file_content)
}

pub fn call_function(spec_file_path: &Path, function_name: &str, args: &[&String]) {
    let mut args_string: String = String::new();
    for arg in args {
        args_string.push_str(&format!("{} ", arg));
    }

    let shell_code = ". {{spec_file_path}} && {{function_name}} {{args}}";
    let shell_code = shell_code
        .replace(
            "{{spec_file_path}}",
            spec_file_path
                .to_str()
                .expect("Could not get str from pathbuf"),
        )
        .replace("{{function_name}}", function_name)
        .replace("{{args}}", &args_string);

    let shell_cmd = cmd!("sh", "-c", shell_code);
    let reader = shell_cmd
        .stderr_to_stdout()
        .stderr_capture()
        .reader()
        .expect("failed to redirect stderr to stdout");

    let lines = BufReader::new(reader).lines();
    for line in lines {
        println!("{}", line.unwrap());
    }
}
