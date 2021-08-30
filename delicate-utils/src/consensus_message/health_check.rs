use crate::prelude::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthCheckPackage {
    pub system_snapshot: SystemSnapshot,
    pub bind_request: service_binding::BindRequest,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemSnapshot {
    // TODO: The heartbeat check is concerned with system metrics
    // And does not require a detailed list of processes.

    // pub processes: Processes,
    pub processor: Processor,
    pub memory: Memory,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Processes {
    inner: HashMap<SysPid, Process>,
}
use std::iter::Iterator;
impl From<&HashMap<SysPid, SysProcess>> for Processes {
    fn from(value: &HashMap<SysPid, SysProcess>) -> Processes {
        let inner: HashMap<SysPid, Process> = value
            .iter()
            .map(|(index, process)| (*index, Into::<Process>::into(process)))
            .collect();

        Processes { inner }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Process {
    pub name: String,
    pub exe: PathBuf,
    pub pid: SysPid,
    pub memory: u64,
    pub virtual_memory: u64,
    pub parent: Option<SysPid>,
    pub start_time: u64,
    pub cpu_usage: f32,
    pub status: u32,
}

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Processor {
    pub cpu_usage: f32,
    pub frequency: u64,
}

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Memory {
    pub total_memory: u64,
    pub used_memory: u64,
    pub free_memory: u64,
}

impl From<&SysProcess> for Process {
    fn from(sys_process: &SysProcess) -> Self {
        let status: u32 = match sys_process.status() {
            SysProcessStatus::Run => 2,
            #[cfg(target_family = "unix")]
            SysProcessStatus::Idle => 1,
            #[cfg(target_family = "unix")]
            SysProcessStatus::Sleep => 3,
            #[cfg(target_family = "unix")]
            SysProcessStatus::Stop => 4,
            #[cfg(target_family = "unix")]
            SysProcessStatus::Zombie => 5,
            #[cfg(target_family = "unix")]
            SysProcessStatus::Unknown(s) => s,
            // Compatible with process states on different systems.
            #[allow(unreachable_patterns)]
            _ => 80,
        };

        Process {
            name: sys_process.name().to_string(),
            exe: sys_process.exe().to_path_buf(),
            pid: sys_process.pid(),
            memory: sys_process.memory(),
            virtual_memory: sys_process.virtual_memory(),
            parent: sys_process.parent(),
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
