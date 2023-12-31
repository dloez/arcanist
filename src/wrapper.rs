use std::path::{Path, PathBuf};

mod bash;
mod python;
mod yaml;

pub struct Wrapper {
    pub wrapper_kind: WrapperKind,
    pub spec_file_path: PathBuf,
}

pub enum WrapperKind {
    Yaml,
    Python,
    Bash,
}

trait WrapperKindActions {
    fn does_function_exists(&self, spec_file_path: &Path, function_name: &str) -> bool;
    fn call_function(&self, spec_file_path: &Path, function_name: &str, args: &[&String]);
}

impl WrapperKindActions for WrapperKind {
    fn does_function_exists(&self, spec_file_path: &Path, function_name: &str) -> bool {
        match self {
            WrapperKind::Yaml => todo!(),
            WrapperKind::Python => python::does_function_exists(spec_file_path, function_name),
            WrapperKind::Bash => bash::does_function_exists(spec_file_path, function_name),
        }
    }

    fn call_function(&self, spec_file_path: &Path, function_name: &str, args: &[&String]) {
        match self {
            WrapperKind::Yaml => todo!(),
            WrapperKind::Python => python::call_function(spec_file_path, function_name, args),
            WrapperKind::Bash => bash::call_function(spec_file_path, function_name, args),
        }
    }
}

pub trait WrapperActions {
    fn print_spec_file_path(&self);
    fn does_function_exists(&self, function_name: &str) -> bool;
    fn call_function(&self, function_name: &str, args: &[&String]);
}

impl WrapperActions for Wrapper {
    fn print_spec_file_path(&self) {
        println!("{:?}", &self.spec_file_path);
    }

    fn does_function_exists(&self, function_name: &str) -> bool {
        self.wrapper_kind
            .does_function_exists(&self.spec_file_path, function_name)
    }

    fn call_function(&self, function_name: &str, args: &[&String]) {
        self.wrapper_kind
            .call_function(&self.spec_file_path, function_name, args)
    }
}
