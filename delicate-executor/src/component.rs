use crate::prelude::*;

/// This is a mirror of the system that can reflect the current state of the system.
#[derive(Debug)]
pub(crate) struct SystemMirror {
    inner_system: RwLock<System>,
    inner_snapshot: RwLock<SystemSnapshot>,
}

impl SystemMirror {
    pub(crate) async fn refresh_all(&self) -> SystemSnapshot {
        let mut system = self.inner_system.write().await;
        system.refresh_cpu();
        system.refresh_memory();
        system.refresh_processes();

        let processes: Processes = system.get_processes().into();
        let processor: Processor = system.get_global_processor_info().into();

        let memory: Memory = Memory {
            total_memory: system.get_total_memory(),
            free_memory: system.get_free_memory(),
            used_memory: system.get_used_memory(),
        };

        let mut inner_snapshot = self.inner_snapshot.write().await;
        inner_snapshot.processes = processes;
        inner_snapshot.processor = processor;
        inner_snapshot.memory = memory;

        inner_snapshot.clone()
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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct SystemSnapshot {
    processes: Processes,
    processor: Processor,
    memory: Memory,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    status: u32,
}

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Processor {
    cpu_usage: f32,
    frequency: u64,
}

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Memory {
    total_memory: u64,
    used_memory: u64,
    free_memory: u64,
}

impl From<&SysProcess> for Process {
    fn from(sys_process: &SysProcess) -> Self {
        let status: u32 = match sys_process.status() {
            SysProcessStatus::Idle => 1,
            SysProcessStatus::Run => 2,
            SysProcessStatus::Sleep => 3,
            SysProcessStatus::Stop => 4,
            SysProcessStatus::Zombie => 5,
            SysProcessStatus::Unknown(s) => s,
            // Compatible with process states on different systems.
            #[allow(unreachable_patterns)]
            _ => 80,
        };

        Process {
            name: sys_process.name().to_string(),
            cmd: sys_process.cmd().to_vec(),
            exe: sys_process.exe().to_path_buf(),
            pid: sys_process.pid() as i32,
            memory: sys_process.memory(),
            virtual_memory: sys_process.virtual_memory(),
            parent: sys_process.parent() as Option<i32>,
            start_time: sys_process.start_time(),
            cpu_usage: sys_process.cpu_usage(),
            status,
        }
    }
}

impl From<&SysProcessor> for Processor {
    fn from(sys_processor: &SysProcessor) -> Self {
        Processor {
            cpu_usage: sys_processor.get_cpu_usage(),
            frequency: sys_processor.get_frequency(),
        }
    }
}
