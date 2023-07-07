use std::path::PathBuf;

mod python;
mod yaml;

pub struct Wrapper {
    pub wrapper_kind: WrapperKind,
    pub spec_file_path: PathBuf,
}

pub enum WrapperKind {
    Yaml,
    Python,
}

trait WrapperKindActions {
    fn does_function_exists(&self, spec_file_path: &PathBuf, function_name: &str) -> bool;
}

impl WrapperKindActions for WrapperKind {
    fn does_function_exists(&self, spec_file_path: &PathBuf, function_name: &str) -> bool {
        match self {
            WrapperKind::Yaml => todo!(),
            WrapperKind::Python => python::does_function_exists(spec_file_path, function_name),
        }
    }
}

pub trait WrapperActions {
    fn print_spec_file_path(&self);
    fn does_function_exists(&self, function_name: &str) -> bool;
}

impl WrapperActions for Wrapper {
    fn print_spec_file_path(&self) {
        println!("{:?}", &self.spec_file_path);
    }

    fn does_function_exists(&self, function_name: &str) -> bool {
        self.wrapper_kind
            .does_function_exists(&self.spec_file_path, function_name)
    }
}
