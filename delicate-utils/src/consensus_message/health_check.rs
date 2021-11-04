pub(crate) use actuator::BindRequest;

use crate::prelude::*;

pub mod proto_health {
    include!("../../proto/generated_codes/delicate.actuator.health.rs");
}
/// This is a mirror of the system that can reflect the current state of the
/// system.
#[derive(Debug)]
pub struct SystemMirror {
    inner_system: AsyncRwLock<System>,
    inner_snapshot: AsyncRwLock<SystemSnapshot>,
}

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

/// An enumeration of values representing gRPC service health.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ServingStatus {
    /// Unknown status
    Unknown,
    /// The service is currently up and serving requests.
    Serving,
    /// The service is currently down and not serving requests.
    NotServing,
}

impl From<&HashMap<SysPid, SysProcess>> for Processes {
    fn from(value: &HashMap<SysPid, SysProcess>) -> Processes {
        let inner: HashMap<SysPid, Process> =
            value.iter().map(|(index, process)| (*index, Into::<Process>::into(process))).collect();

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

        Process { name: sys_process.name().to_string(),
                  exe: sys_process.exe().to_path_buf(),
                  pid: sys_process.pid(),
                  memory: sys_process.memory(),
                  virtual_memory: sys_process.virtual_memory(),
                  parent: sys_process.parent(),
                  start_time: sys_process.start_time(),
                  cpu_usage: sys_process.cpu_usage(),
                  status }
    }
}

impl fmt::Display for ServingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServingStatus::Unknown => f.write_str("Unknown"),
            ServingStatus::Serving => f.write_str("Serving"),
            ServingStatus::NotServing => f.write_str("NotServing"),
        }
    }
}

impl From<ServingStatus> for proto_health::health_check_response::ServingStatus {
    fn from(s: ServingStatus) -> Self {
        match s {
            ServingStatus::Unknown => proto_health::health_check_response::ServingStatus::Unknown,
            ServingStatus::Serving => proto_health::health_check_response::ServingStatus::Serving,
            ServingStatus::NotServing => {
                proto_health::health_check_response::ServingStatus::NotServing
            },
        }
    }
}

impl From<SystemSnapshot> for proto_health::SystemSnapshot {
    fn from(SystemSnapshot { processor, memory }: SystemSnapshot) -> proto_health::SystemSnapshot {
        proto_health::SystemSnapshot { processor: Some(processor.into()),
                                       memory: Some(memory.into()) }
    }
}

impl From<Processor> for proto_health::system_snapshot::Processor {
    fn from(Processor { cpu_usage, frequency }: Processor)
            -> proto_health::system_snapshot::Processor {
        proto_health::system_snapshot::Processor { cpu_usage, frequency }
    }
}

impl From<Memory> for proto_health::system_snapshot::Memory {
    fn from(Memory { total_memory, used_memory, free_memory }: Memory)
            -> proto_health::system_snapshot::Memory {
        proto_health::system_snapshot::Memory { total_memory, used_memory, free_memory }
    }
}

impl From<&SysProcessor> for Processor {
    fn from(sys_processor: &SysProcessor) -> Self {
        Processor { cpu_usage: sys_processor.get_cpu_usage(),
                    frequency: sys_processor.get_frequency() }
    }
}

impl SystemMirror {
    pub async fn refresh_all(&self) -> SystemSnapshot {
        let mut system = self.inner_system.write().await;
        system.refresh_cpu();
        system.refresh_memory();

        // TODO: The heartbeat check is concerned with system metrics
        // And does not require a detailed list of processes.
        // system.refresh_processes();
        // let processes: Processes = system.get_processes().into();

        let processor: Processor = system.get_global_processor_info().into();

        let memory: Memory = Memory { total_memory: system.get_total_memory(),
                                      free_memory: system.get_available_memory(),
                                      used_memory: system.get_used_memory() };

        let mut inner_snapshot = self.inner_snapshot.write().await;
        // inner_snapshot.processes = processes;
        inner_snapshot.processor = processor;
        inner_snapshot.memory = memory;

        inner_snapshot.clone()
    }
}

impl Default for SystemMirror {
    fn default() -> SystemMirror {
        let inner_system = AsyncRwLock::new(System::new_with_specifics(
            RefreshKind::new()
                .without_users_list()
                .without_components()
                .with_components_list()
                .without_networks()
                .without_networks_list(),
        ));
        let inner_snapshot = AsyncRwLock::new(SystemSnapshot::default());

        SystemMirror { inner_system, inner_snapshot }
    }
}
