#![cfg_attr(unstable_never_type, feature(never_type))]
#![cfg_attr(unstable_try_trait_v2, feature(try_trait_v2))]
#![cfg_attr(unstable_try_trait_v2_residual, feature(try_trait_v2_residual))]

use std::{
    fmt::Debug,
    io,
    process::{Child, Output, Termination as _T},
};

use bitflags::bitflags;
use clap::{Parser, Subcommand};
use clap_cargo::style::CLAP_STYLING as CARGO_STYLING;
use exit_safely::Termination;
use try_v2::Try;

pub mod commands;

#[derive(Debug, Termination, Try, PartialEq, PartialOrd, Eq, Ord)]
#[FromResidual(Result<_, Self::Residual>)]
#[repr(u8)]
#[must_use]
pub enum Exit<T: _T> {
    Ok(T) = 0,
    Error(String) = 1,
    InvocationError(String) = 2,
    IO(String) = 3,
}

impl Exit<()> {
    fn message(&self) -> &str {
        match self {
            Exit::Ok(_) => "",
            Exit::Error(m) => m,
            Exit::InvocationError(m) => m,
            Exit::IO(m) => m,
        }
    }

    fn replace_message(self, msg: String) -> Option<Self> {
        match self {
            Exit::Ok(_) => None,
            Exit::Error(_) => Some(Exit::Error(msg)),
            Exit::InvocationError(_) => Some(Exit::InvocationError(msg)),
            Exit::IO(_) => Some(Exit::IO(msg)),
        }
    }
}

impl FromIterator<Exit<()>> for Exit<()> {
    fn from_iter<I: IntoIterator<Item = Exit<()>>>(iter: I) -> Self {
        let mut msg = String::new();
        iter.into_iter()
            .filter_map(|e| {
                if let Exit::Ok(_) = e {
                    None
                } else {
                    msg.push_str(e.message());
                    msg.push('\n');
                    Some(e)
                }
            })
            .min()
            .and_then(|e| e.replace_message(msg))
            .unwrap_or(Exit::Ok(()))
    }
}

impl<T: _T> From<clap::Error> for Exit<T> {
    fn from(e: clap::Error) -> Self {
        Self::InvocationError(e.to_string())
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

impl From<Cmd> for Exit<()> {
    fn from(cmd: Cmd) -> Self {
        let flags = cmd.flags;
        let task = cmd.name;
        match cmd.result {
            Ok(output) if flags.contains(CheckFlags::JSON) => {
                let status = output.status;
                let payload = String::from_utf8_lossy(&output.stdout);
                let mut json = String::new();
                json.push_str(r#"{ "task": "#);
                json.push('"');
                json.push_str(task);
                json.push('"');
                json.push_str(r#", "status": "#);
                json.push('"');
                json.push_str(&status.to_string());
                json.push('"');
                if !status.success() {
                    json.push_str(r#", "payload": "#);
                    json.push_str(&payload);
                }
                json.push('}');
                println!("{json}");
                if status.success() {
                    Exit::Ok(())
                } else {
                    Exit::Error(String::new())
                }
            }
            Ok(output) if !output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                Self::Error(format!(
                    "====== {task} exited with {status} ======\n-- stdout: --\n{stdout}\n\n-- stderr: --\n{stderr}",
                    status = output.status
                ))
            }
            Ok(_) => {
                println!("{task}: OK");
                Self::Ok(())
            }
            Err(err_spawning) if flags.contains(CheckFlags::JSON) => {
                let mut json = String::new();
                json.push_str(r#"{ "task": "#);
                json.push('"');
                json.push_str(task);
                json.push('"');
                json.push_str(r#", "status": "failed to spawn""#);
                json.push_str(r#", "error": "#);
                json.push('"');
                json.push_str(&err_spawning.to_string());
                json.push('"');
                json.push('}');
                println!("{json}");
                Exit::IO(String::new())
            }
            Err(err_spawning) => {
                let msg = format!("{task} failed: {err_spawning}");
                Self::IO(msg)
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

impl FromIterator<Spawned> for Exit<()> {
    fn from_iter<I: IntoIterator<Item = Spawned>>(spawns: I) -> Self {
        spawns.into_iter().map(Exit::from).collect()
    }
}

impl From<Spawned> for Exit<()> {
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
    use std::process::Command;

    use super::*;

    #[test]
    fn exit_from_404() {
        let splat: Cmd = Command::new("splat").output().into_cmd("splat", None);
        assert_eq!(splat.name, "splat");
        assert!(
            matches!(splat.result, Result::Err(ref e) if matches!(e.kind(), io::ErrorKind::NotFound))
        );
        let exit: Exit<()> = Exit::from(splat);
        let Exit::IO(ref msg) = exit else {
            panic!("not an IO2")
        };
        eprintln!("{}", msg);
        assert!(msg.starts_with("splat failed: "));
    }

    #[test]
    fn collect_exit() {
        let exits = [
            Exit::Ok(()),
            Exit::IO("one".to_string()),
            Exit::Error("two".to_string()),
            Exit::Error("three".to_string()),
        ];
        let exit: Exit<()> = exits.into_iter().collect();
        let expected = "one\ntwo\nthree\n";
        dbg!(&exit);
        assert!(matches!(exit, Exit::Error(s) if s == expected));
    }
}
