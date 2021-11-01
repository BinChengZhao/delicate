pub use prost_types::Any;

use crate::uniform_data::Trial;

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

impl UnifiedResponseMessagesForGrpc {
    #[inline(always)]
    pub fn success_with_data(data: Vec<Any>) -> Self {
        UnifiedResponseMessagesForGrpc { data, ..Default::default() }
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn error_with_data(data: Vec<Any>) -> Self {
        let code = -1;
        let msg = String::default();
        UnifiedResponseMessagesForGrpc { code, msg, data }
    }

    #[inline(always)]
    pub fn customized_error_msg(mut self, msg: String) -> Self {
        self.msg = msg;

        self
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn customized_error_code(mut self, code: i32) -> Self {
        self.code = code;

        self
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn reverse(mut self) -> Self {
        self.code = -1 - self.code;
        self
    }

    #[inline(always)]
    pub fn get_data(self) -> Vec<Any> {
        self.data
    }

    #[inline(always)]
    pub fn get_data_ref(&self) -> &[Any] {
        &self.data
    }
}

impl UnifiedResponseMessagesForGrpc {
    #[inline(always)]
    pub fn error() -> Self {
        UnifiedResponseMessagesForGrpc { code: -1, ..Default::default() }
    }

    #[inline(always)]
    pub fn success() -> Self {
        UnifiedResponseMessagesForGrpc { code: 0, ..Default::default() }
    }
}

impl Trial for UnifiedResponseMessagesForGrpc {
    #[inline(always)]
    fn get_msg(&self) -> String {
        self.msg.clone()
    }

    #[inline(always)]
    fn is_err(&self) -> bool {
        self.code != 0
    }

    #[inline(always)]
    fn is_ok(&self) -> bool {
        self.code == 0
    }
}
