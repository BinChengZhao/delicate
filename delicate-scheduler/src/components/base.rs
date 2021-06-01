use super::security::SchedulerSecurityConf;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SchedulerMetaInfo {
    name: String,
    domin: String,
    listening_address: String,
    security_conf: SchedulerSecurityConf,
}

impl Default for SchedulerMetaInfo {
    fn default() -> Self {
        todo!();
    }
}
