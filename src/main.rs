use clap::{arg, Command};
use glob::glob;
use std::{env, path::PathBuf};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod wrapper;
use wrapper::WrapperActions;

#[derive(EnumIter)]
enum SupportedExtension {
    Yaml,
    Python,
    Bash,
}

trait Extension {
    fn get_extensions_str(&self) -> Vec<&str>;
    fn get_wrapper(&self, spec_file_path: PathBuf) -> wrapper::Wrapper;
}

impl Extension for SupportedExtension {
    fn get_extensions_str(&self) -> Vec<&str> {
        match self {
            SupportedExtension::Yaml => vec!["yaml", "yml"],
            SupportedExtension::Python => vec!["py"],
            SupportedExtension::Bash => vec!["sh"],
        }
    }

    fn get_wrapper(&self, spec_file_path: PathBuf) -> wrapper::Wrapper {
        match self {
            SupportedExtension::Yaml => {
                let wrapper_kind = wrapper::WrapperKind::Yaml;
                wrapper::Wrapper {
                    spec_file_path,
                    wrapper_kind,
                }
            }
            SupportedExtension::Python => {
                let wrapper_kind = wrapper::WrapperKind::Python;
                wrapper::Wrapper {
                    spec_file_path,
                    wrapper_kind,
                }
            }
            SupportedExtension::Bash => {
                let wrapper_kind = wrapper::WrapperKind::Bash;
                wrapper::Wrapper {
                    spec_file_path,
                    wrapper_kind,
                }
            }
        }
    }
}

fn get_spec_files_paths() -> Vec<wrapper::Wrapper> {
    let mut wrappers = Vec::new();
    for supported_extension in SupportedExtension::iter() {
        for extension in supported_extension.get_extensions_str() {
            let mut found = false;
            let pattern = format!("**/arcanist.{}", extension);
            for entry in glob(&pattern).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        wrappers.push(supported_extension.get_wrapper(path));
                        found = true;
                    }
                    _ => unimplemented!(),
                };
            }

            if found {
                break;
            }
        }
    }
    wrappers
}

fn main() {
    let version: &'static str = env!("CARGO_PKG_VERSION");
    let matches = Command::new("Arcanist")
        .version(version)
        .author("David L. <davidlopez.hellin@outlook.com>")
        .about("A 'Mage' inspired function runner with multi-language support")
        .arg(arg!([function_name] "Function to call").required(true))
        .arg(
            arg!([args] "Arguments to pass to the function")
                .required(false)
                .num_args(0..)
                .default_missing_value("")
                .default_value(""),
        )
        .get_matches();

    let wrappers = get_spec_files_paths();
    let function_name = matches
        .get_one::<String>("function_name")
        .expect("required");
    let args: Vec<&String> = matches.get_many::<String>("args").unwrap().collect();
    for wrapper in wrappers {
        if wrapper.does_function_exists(function_name) {
            wrapper.call_function(function_name, &args);
        };
    }
}
