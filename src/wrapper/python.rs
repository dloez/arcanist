use std::{fs, path::PathBuf};

use regex::Regex;

const REGEX_FUNCTION_PATTERN: &str = r"def[ \t]*{{function_name}}\b.*:";

pub fn does_function_exists(spec_file_path: &PathBuf, function_name: &str) -> bool {
    let file_content =
        fs::read_to_string(spec_file_path).expect(&format!("Failed to read {:?}", spec_file_path));

    let regex_pattern = &REGEX_FUNCTION_PATTERN.replace("{{function_name}}", function_name);
    let regex = Regex::new(&regex_pattern).expect("Failed to define regex");
    regex.is_match(&file_content)
}
