use std::{env::VarError, ffi::OsString};

/// Result type wrapping [BuildError]. Returning this from `main` in a build script will
/// provide useful information in the debug representation sent to stderr on failure.
pub type Result<T> = std::result::Result<T, BuildError>;

/// Attempt to get an environment variable. If not found the error returned will include the
/// key name in the debug representation.
pub fn get_var(key: &str) -> Result<String> {
    std::env::var(key).map_err(|err| BuildError::from_var_error(key, err))
}

#[derive(Debug)]
/// An error designed to have nice debug representations for common errors encountered
/// in build.rs
pub enum BuildError {
    VarNotSet(OsString),
    VarInvalid(OsString, OsString),
    IOError(Box<std::io::Error>),
    Other(String),
}

impl BuildError {
    /// Ensure that the key name is included in the error returned.
    /// You probably won't need this often and can use [get_var] for most cases.
    pub fn from_var_error(key: &str, err: VarError) -> BuildError {
        match err {
            VarError::NotPresent => BuildError::VarNotSet(key.into()),
            VarError::NotUnicode(contents) => BuildError::VarInvalid(key.into(), contents),
        }
    }
}

impl From<std::io::Error> for BuildError {
    fn from(e: std::io::Error) -> Self {
        BuildError::IOError(Box::new(e))
    }
}

/// Generate your own with `Err("some text")`
impl From<&'static str> for BuildError {
    fn from(msg: &'static str) -> Self {
        msg.to_string().into()
    }
}

impl From<String> for BuildError {
    fn from(msg: String) -> Self {
        BuildError::Other(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn err_from_str() {
        let err = BuildError::from("oops");
        let msg = r#"Other("oops")"#;
        assert!(format!("{err:?}").contains(msg));
    }

    #[test]
    fn missing_env_var() {
        let random_key = "019de8d0-bb66-769d-9d4d-fec48aebdd49";
        let err = get_var(random_key);
        assert!(err.is_err());
        assert!(format!("{err:?}").contains(random_key));
    }
}
