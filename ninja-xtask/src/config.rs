//! Load custom config from `[workspace.metadata.ninja-xtask]` in `Cargo.toml`

use serde_json::{Map, Value};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Config {
    env: Map<String, Value>,
}

impl From<Value> for Config {
    fn from(metadata: Value) -> Self {
        let our_config = metadata
            .as_object()
            .and_then(|metadata| metadata.get("ninja-xtask"));
        match our_config {
            Some(config) => Self {
                env: config
                    .as_object()
                    .and_then(|config| config.get("env"))
                    .and_then(|env| env.as_object())
                    .cloned()
                    .unwrap_or_default(),
            },
            None => Self::default(),
        }
    }
}

impl Config {
    pub fn envs(&self) -> impl IntoIterator<Item = (&String, &str)> {
        self.env
            .iter()
            .filter_map(|(key, value)| value.as_str().map(|value| (key, value)))
    }

    pub fn env_remove(&self) -> impl IntoIterator<Item = &String> {
        self.env
            .iter()
            .filter_map(|(key, value)| value.as_bool().filter(|value| !value).map(|_| key))
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;

    use serde_json::json;

    use super::*;

    #[test]
    fn envs() {
        let metadata = json!({
            "ninja-xtask": {
                "env": {
                    "LD_LIBRARY_PATH": false,
                    "CARGO_INCREMENTAL": "1"
                }
            }
        });
        let config = Config::from(metadata);
        let vars: Vec<_> = config.envs().into_iter().collect();
        let expected = [(&"CARGO_INCREMENTAL".to_string(), "1")];
        assert_eq!(vars, expected);
        let _assert_os_strs: Vec<(&OsStr, &OsStr)> = vars
            .into_iter()
            .map(|(k, v)| (k.as_ref(), v.as_ref()))
            .collect();
    }

    #[test]
    fn env_remove() {
        let metadata = json!({
            "ninja-xtask": {
                "env": {
                    "LD_LIBRARY_PATH": false,
                    "CARGO_INCREMENTAL": "1"
                }
            }
        });
        let config = Config::from(metadata);
        let var = config.env_remove().into_iter().next().unwrap();
        let expected = "LD_LIBRARY_PATH";
        assert_eq!(var, expected);
        let _assert_os_str: &OsStr = var.as_ref();
    }
}
