#![cfg_attr(unstable_never_type, feature(never_type))]
#![cfg_attr(unstable_try_trait_v2, feature(try_trait_v2))]
#![cfg_attr(unstable_try_trait_v2_residual, feature(try_trait_v2_residual))]

use std::{
    fmt::{Debug, Display},
    io,
    process::{Child, Output, Termination as _T},
};

use bitflags::bitflags;
use clap::{Parser, Subcommand};
use clap_cargo::style::CLAP_STYLING as CARGO_STYLING;
use exit_safely::Termination;
use serde_json::{
    Value::{self, Array},
    json,
};
use try_v2::Try;

pub mod commands;

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

#[derive(Debug, PartialEq, Eq, Default)]
pub struct WithJson<T> {
    pub value: T,
    pub json: Option<Value>,
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

#[derive(Debug)]
pub struct Cmd {
    pub name: &'static str,
    pub result: Result<Output, io::Error>,
    pub flags: CheckFlags,
}

trait CmdExt {
    fn into_cmd(self, name: &'static str, checkflags: Option<CheckFlags>) -> Cmd;
}

impl CmdExt for Result<Output, io::Error> {
    fn into_cmd(self, name: &'static str, checkflags: Option<CheckFlags>) -> Cmd {
        Cmd {
            name,
            result: self,
            flags: checkflags.unwrap_or_default(),
        }
    }
}

impl From<Cmd> for Exit<WithJson<()>> {
    fn from(cmd: Cmd) -> Self {
        let flags = cmd.flags;
        let task = cmd.name;
        match cmd.result {
            Ok(output) if flags.contains(CheckFlags::JSON) => {
                let status = output.status;
                let payload = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .filter(|line| line.starts_with("{"))
                    .map(serde_json::from_str::<Value>)
                    .map(|json| json.unwrap_or_else(|err| json!({"unparsable": &err.to_string()})))
                    .collect::<Value>();
                let json = json!({
                    "task": task,
                    "status": &status.to_string(),
                    "payload": payload,
                });
                if status.success() {
                    Exit::Ok(WithJson {
                        value: (),
                        json: Some(json),
                    })
                } else {
                    Exit::Error(WithJson {
                        value: String::new(),
                        json: Some(json),
                    })
                }
            }
            Ok(output) if !output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                Self::Error(WithJson {
                    value: format!(
                        "====== {task} exited with {status} ======\n-- stdout: --\n{stdout}\n\n-- stderr: --\n{stderr}",
                        status = output.status
                    ),
                    json: None,
                })
            }
            Ok(_) => {
                println!("{task}: OK");
                Self::Ok(WithJson {
                    value: (),
                    json: None,
                })
            }
            Err(err_spawning) if flags.contains(CheckFlags::JSON) => {
                let json = json!({
                    "task": task,
                    "status": "failed to spawn",
                    "error": &err_spawning.to_string(),
                });
                Exit::IO(WithJson {
                    value: String::new(),
                    json: Some(json),
                })
            }
            Err(err_spawning) => {
                let msg = format!("{task} failed: {err_spawning}");
                Self::IO(WithJson {
                    value: msg,
                    json: None,
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct Spawned {
    pub name: &'static str,
    pub child: Result<Child, io::Error>,
    pub flags: CheckFlags,
}

impl Spawned {
    pub fn wait(self) -> Cmd {
        match self.child {
            Ok(child) => child
                .wait_with_output()
                .into_cmd(self.name, Some(self.flags)),
            Err(e) => Cmd {
                name: self.name,
                result: Err(e),
                flags: self.flags,
            },
        }
    }
}

trait SpawnedExt {
    fn into_spawned(self, name: &'static str, flags: Option<CheckFlags>) -> Spawned;
}

impl SpawnedExt for Result<Child, io::Error> {
    fn into_spawned(self, name: &'static str, flags: Option<CheckFlags>) -> Spawned {
        Spawned {
            name,
            child: self,
            flags: flags.unwrap_or_default(),
        }
    }
}

impl FromIterator<Spawned> for Exit<WithJson<()>> {
    fn from_iter<I: IntoIterator<Item = Spawned>>(spawns: I) -> Self {
        spawns.into_iter().map(Exit::from).collect()
    }
}

impl From<Spawned> for Exit<WithJson<()>> {
    fn from(spawn: Spawned) -> Self {
        spawn.wait().into()
    }
}

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
#[command(styles = CARGO_STYLING)]
pub enum CargoCmd {
    #[command(subcommand)]
    Ninja(NinjaCommand),
}

#[derive(Subcommand)]
#[command(version)]
pub enum NinjaCommand {
    /// fmt, lint & test then stage everything in git if all is good
    Stage {
        /// add --deny warnings to clippy invocations
        #[arg(long)]
        strict: bool,
        /// output in json format
        #[arg(long)]
        json: bool,
    },
    /// build (optionally with zigbuild for a given glibc version)
    Build {
        /// build for a specific glibc version (WSL-Ubuntu is 2.35)
        #[arg(short, long)]
        glibc: Option<String>,
        /// build a release build (default is cargo's default profile, usually debug)
        #[arg(short, long)]
        release: bool,
        /// build for a given target
        #[arg(long)]
        target: Option<String>,
    },
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CheckFlags: u32 {
        const STRICT = 0b00000001;
        const JSON = 0b00000010;
    }
}

impl From<&NinjaCommand> for CheckFlags {
    fn from(xtask: &NinjaCommand) -> Self {
        match xtask {
            NinjaCommand::Stage { strict, json } => {
                let mut flags = Self::default();
                flags.set(Self::STRICT, *strict);
                flags.set(Self::JSON, *json);
                flags
            }
            NinjaCommand::Build { .. } => CheckFlags::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{assert_matches, process::Command};

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
