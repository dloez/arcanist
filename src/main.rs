use clap::{arg, Command};
use glob::glob;
use std::path::PathBuf;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use wrapper::WrapperActions;

mod wrapper;

#[derive(EnumIter)]
enum SupportedExtension {
    Yaml,
    Python,
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
    let matches = Command::new("Arcanist")
        .version("0.1")
        .author("David L. <davidlopez.hellin@outlook.com>")
        .about("A 'Mage' like inspired function runner with multi-language support")
        .arg(arg!([function_name] "Function to call").required(true))
        .get_matches();

    let wrappers = get_spec_files_paths();
    let function_name = matches
        .get_one::<String>("function_name")
        .expect("required");
    for wrapper in wrappers {
        println!("{:?}", wrapper.does_function_exists(function_name));
    }
}
