use std::{
    fmt::{Debug, Display},
    io,
    process::Termination as _T,
};

use exit_safely::Termination;
use serde_json::Value::{self, Array};
use try_v2::Try;

#[derive(Debug, Termination, Try, PartialEq, PartialOrd, Eq, Ord)]
#[FromResidual(Result<_, Self::Residual>)]
#[repr(u8)]
#[must_use]
pub enum Exit<T: _T> {
    Ok(T) = 0,
    Error(WithJson<String>) = 1,
    InvocationError(WithJson<String>) = 2,
    IO(WithJson<String>) = 3,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct WithJson<T> {
    pub value: T,
    pub json: Option<Value>,
}

impl<T> _T for WithJson<T>
where
    T: _T,
{
    fn report(self) -> std::process::ExitCode {
        if let Some(json) = self.json {
            println!("{json}");
        };
        self.value.report()
    }
}

impl<T> Display for WithJson<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)?;
        if let Some(json) = self.json.clone() {
            write!(f, "\n{}", json)?;
        };
        Ok(())
    }
}

impl<T> Ord for WithJson<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.value.cmp(&other.value) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        Ord::cmp(
            &self.json.as_ref().unwrap_or_default().to_string(),
            &other.json.as_ref().unwrap_or_default().to_string(),
        )
    }
}

impl<T> PartialOrd for WithJson<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.value.partial_cmp(&other.value) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        PartialOrd::partial_cmp(
            &self.json.as_ref().unwrap_or_default().to_string(),
            &other.json.as_ref().unwrap_or_default().to_string(),
        )
    }
}

impl Exit<WithJson<()>> {
    fn message(&self) -> &str {
        match self {
            Exit::Ok(_) => "",
            Exit::Error(WithJson {
                value: msg,
                json: _,
            }) => msg,
            Exit::InvocationError(WithJson {
                value: msg,
                json: _,
            }) => msg,
            Exit::IO(WithJson {
                value: msg,
                json: _,
            }) => msg,
        }
    }

    fn replace_message(self, msg: String, jsons: Vec<Value>) -> Self {
        let json = (!jsons.is_empty()).then(|| jsons.into_iter().collect::<Value>());
        match self {
            Exit::Ok(_) => Self::Ok(WithJson { value: (), json }),
            Exit::Error(_) => Exit::Error(WithJson { value: msg, json }),
            Exit::InvocationError(_) => Exit::InvocationError(WithJson { value: msg, json }),
            Exit::IO(_) => Exit::IO(WithJson { value: msg, json }),
        }
    }

    pub fn take_json(&mut self) -> Option<Value> {
        match self {
            Exit::Ok(WithJson { json, .. }) => json.take(),
            Exit::Error(WithJson { json, .. }) => json.take(),
            Exit::InvocationError(WithJson { json, .. }) => json.take(),
            Exit::IO(WithJson { json, .. }) => json.take(),
        }
    }
}

impl FromIterator<Exit<WithJson<()>>> for Exit<WithJson<()>> {
    fn from_iter<I: IntoIterator<Item = Exit<WithJson<()>>>>(iter: I) -> Self {
        let mut msg = String::new();
        let mut jsons = Vec::<Value>::new();
        iter.into_iter()
            .map(|mut exit| {
                msg.push_str(exit.message());
                match exit.take_json() {
                    Some(Array(json)) => jsons.extend(json),
                    Some(json) => jsons.push(json),
                    None => {}
                }
                exit
            })
            .max()
            .map(|highest_exit_code| highest_exit_code.replace_message(msg, jsons))
            .unwrap_or(Exit::Ok(Default::default()))
    }
}

impl<T: _T> From<clap::Error> for Exit<T> {
    fn from(e: clap::Error) -> Self {
        Self::InvocationError(WithJson {
            value: e.to_string(),
            json: None,
        })
    }
}

impl<T: _T> From<io::Error> for Exit<T> {
    fn from(e: io::Error) -> Self {
        Self::IO(WithJson {
            value: e.to_string(),
            json: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{assert_matches, io, process::Command};

    use crate::commands::{Cmd, CmdExt as _};

    use super::*;

    #[test]
    fn exit_from_404() {
        let splat: Cmd = Command::new("splat").output().into_cmd("splat", None);
        assert_eq!(splat.name, "splat");
        assert!(
            matches!(splat.result, Result::Err(ref e) if matches!(e.kind(), io::ErrorKind::NotFound))
        );
        let exit: Exit<WithJson<()>> = Exit::from(splat);
        let Exit::IO(WithJson {
            value: msg,
            json: _,
        }) = exit
        else {
            panic!("not an IO2")
        };
        eprintln!("{}", msg);
        assert!(msg.starts_with("splat failed: "));
    }

    #[test]
    fn collect_exit() {
        let exits = [
            Exit::Ok(WithJson {
                value: (),
                json: None,
            }),
            Exit::IO(WithJson {
                value: "one\n".to_string(),
                json: None,
            }),
            Exit::Error(WithJson {
                value: "two\n".to_string(),
                json: None,
            }),
            Exit::Error(WithJson {
                value: "three\n".to_string(),
                json: None,
            }),
        ];
        let exit: Exit<WithJson<()>> = exits.into_iter().collect();
        let expected = "one\ntwo\nthree\n";
        assert_matches!(exit, Exit::IO(s) if s.value == expected);
    }
}
