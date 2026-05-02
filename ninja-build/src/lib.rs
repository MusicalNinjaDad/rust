use std::{env::VarError, ffi::OsString};

pub type Result<T> = std::result::Result<T, BuildError>;

pub fn get_var(key: &str) -> Result<String> {
    std::env::var(key).map_err(|e| BuildError::from_var_error(key, e))
}

#[derive(Debug)]
pub enum BuildError {
    VarNotSet(OsString),
    VarInvalid(OsString, OsString),
    IOError(Box<std::io::Error>),
}

impl BuildError {
    fn from_var_error(var: &str, e: VarError) -> BuildError {
        match e {
            VarError::NotPresent => BuildError::VarNotSet(var.into()),
            VarError::NotUnicode(contents) => BuildError::VarInvalid(var.into(), contents),
        }
    }
}

impl From<std::io::Error> for BuildError {
    fn from(e: std::io::Error) -> Self {
        BuildError::IOError(Box::new(e))
    }
}
