use actix_web::web::Data as ShareData;

use delay_timer::prelude::*;

use serde::{Deserialize, Serialize};

use sysinfo::{Process as SysProcess, RefreshKind, System, SystemExt};

use async_lock::RwLock;

use std::collections::HashMap;
use std::convert::From;
use std::fmt::Debug;
use std::path::PathBuf;

pub(crate) type SharedDelayTimer = ShareData<DelayTimer>;

/// This is a mirror of the system that can reflect the current state of the system.
// TODO:
#[derive(Debug)]
pub(crate) struct SystemMirror {
    inner_system: RwLock<System>,
    inner_snapshot: RwLock<SystemSnapshot>,
}

impl SystemMirror {
    pub(crate) async fn refresh_all(&self) -> SystemSnapshot {
        todo!();
        // let inner_processes: &HashMap<usize, SysProcess>;
        // let processes: Processes;

        // {
        //     let mut system = self.inner_system.write().await;
        //     system.refresh_all();
        //     inner_processes = system.get_processes();
        //     processes = inner_processes.into();
        // }

        // let mut inner_snapshot = self.inner_snapshot.write().await;
        // inner_snapshot.processes = processes;

        // inner_snapshot.clone()
    }
}

impl Default for SystemMirror {
    fn default() -> SystemMirror {
        let inner_system = RwLock::new(System::new_with_specifics(
            RefreshKind::everything()
                .without_components()
                .without_components_list()
                .without_users_list(),
        ));
        SystemMirror {
            inner_system,
            ..Default::default()
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub(crate) struct SystemSnapshot {
    processes: Processes,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct Processes {
    inner: HashMap<i32, Process>,
}
use std::iter::Iterator;
impl From<&HashMap<i32, SysProcess>> for Processes {
    fn from(value: &HashMap<i32, SysProcess>) -> Processes {
        let inner: HashMap<i32, Process> = value
            .iter()
            .map(|(index, process)| (*index, Into::<Process>::into(process)))
            .collect();

        Processes { inner }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct Process {
    name: String,
    cmd: Vec<String>,
    exe: PathBuf,
    pid: i32,
    memory: u64,
    virtual_memory: u64,
    parent: Option<i32>,
    start_time: u64,
    cpu_usage: f32,
    //TODO: ProcessStatus should be stored in Process;
}

impl From<&SysProcess> for Process {
    fn from(_sys_process: &SysProcess) -> Self {
        todo!();
        // Process {
        //     name: sys_process.name().to_string(),
        //     cmd: sys_process.cmd().to_vec(),
        //     exe: sys_process.exe().to_path_buf(),
        //     pid: sys_process.pid() as i32,
        //     memory: sys_process.memory(),
        //     virtual_memory: sys_process.virtual_memory(),
        //     parent: sys_process.parent() as i32,
        //     start_time: sys_process.start_time(),
        //     cpu_usage: sys_process.cpu_usage(),
        // }
    }
}
