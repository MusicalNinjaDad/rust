use std::{
    fmt::Debug,
    io,
    process::{Child, Output},
};

use serde_json::{Value, json};

mod stage;
pub use stage::*;

mod build;
pub use build::*;

use crate::{CheckFlags, Exit, WithJson};

#[derive(Debug)]
pub struct Cmd {
    pub name: &'static str,
    pub result: Result<Output, io::Error>,
    pub flags: CheckFlags,
}

pub trait CmdExt {
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
        let Cmd {
            name: task,
            result: did_it_spawn,
            flags,
        } = cmd;

        let output = match did_it_spawn {
            Ok(output) => output,
            Err(err_spawning) => {
                let json = flags.contains(CheckFlags::JSON).then(|| {
                    json!({
                        "task": task,
                        "status": "failed to spawn",
                        "error": &err_spawning.to_string(),
                    })
                });
                let msg = format!("{task} failed: {err_spawning}");
                return match json {
                    Some(json) => Exit::IO(WithJson {
                        value: String::new(),
                        json: Some(json),
                    }),
                    None => Self::IO(WithJson {
                        value: msg,
                        json: None,
                    }),
                };
            }
        };

        let status = output.status;
        let json = flags.contains(CheckFlags::JSON).then(|| {
            let payload = String::from_utf8_lossy(&output.stdout)
                .lines()
                .filter(|line| line.starts_with("{"))
                .map(serde_json::from_str::<Value>)
                .map(|json| json.unwrap_or_else(|err| json!({"unparsable": &err.to_string()})))
                .collect::<Value>();
            json!({
                "task": task,
                "status": &status.to_string(),
                "payload": payload,
            })
        });
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        match (status.success(), json) {
            (true, Some(json)) => Exit::Ok(WithJson {
                value: (),
                json: Some(json),
            }),
            (true, None) => {
                println!("{task}: OK");
                Self::Ok(WithJson {
                    value: (),
                    json: None,
                })
            }
            (false, Some(json)) => Exit::Error(WithJson {
                value: String::new(),
                json: Some(json),
            }),
            (false, None) => Self::Error(WithJson {
                value: format!(
                    "====== {task} exited with {status} ======\n-- stdout: --\n{stdout}\n\n-- stderr: --\n{stderr}",
                    status = output.status
                ),
                json: None,
            }),
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

pub trait SpawnedExt {
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
