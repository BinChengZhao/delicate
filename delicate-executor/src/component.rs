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

        // TODO: The heartbeat check is concerned with system metrics
        // And does not require a detailed list of processes.
        // system.refresh_processes();
        // let processes: Processes = system.get_processes().into();

        let processor: Processor = system.get_global_processor_info().into();

        let memory: Memory = Memory {
            total_memory: system.get_total_memory(),
            free_memory: system.get_available_memory(),
            used_memory: system.get_used_memory(),
        };

        let mut inner_snapshot = self.inner_snapshot.write().await;
        // inner_snapshot.processes = processes;
        inner_snapshot.processor = processor;
        inner_snapshot.memory = memory;

        inner_snapshot.clone()
    }
}

impl Default for SystemMirror {
    fn default() -> SystemMirror {
        let inner_system = RwLock::new(System::new_with_specifics(
            RefreshKind::new()
                .without_users_list()
                .without_components()
                .with_components_list()
                .without_networks()
                .without_networks_list(),
        ));
        let inner_snapshot = RwLock::new(SystemSnapshot::default());

        SystemMirror {
            inner_system,
            inner_snapshot,
        }
    }
}
