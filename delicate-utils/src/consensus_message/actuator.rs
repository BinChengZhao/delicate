include!("../../proto/generated_codes/delicate.actuator.rs");

impl Task {
    fn set_task_id(mut self, id: u64) -> Self {
        self.id = id;
        self
    }

    fn set_task_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    fn set_command(mut self, command: String) -> Self {
        self.command = command;
        self
    }
}
