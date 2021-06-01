use super::security::SchedulerSecurityConf;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SchedulerMetaInfo {
    name: String,
    domain: String,
    listening_address: String,
    security_conf: SchedulerSecurityConf,
}

impl Default for SchedulerMetaInfo {
    fn default() -> Self {
        let domain = env::var_os("SCHEDULER_DOMAIN")
            .expect("No environment variable set `SCHEDULER_DOMAIN`.")
            .to_str()
            .map(|s| s.to_owned())
            .expect("Environment variable resolution failure.");

        let name = env::var_os("SCHEDULER_NAME")
            .expect("No environment variable set `SCHEDULER_NAME`.")
            .to_str()
            .map(|s| s.to_owned())
            .expect("Environment variable resolution failure.");

        let listening_address = env::var_os("SCHEDULER_LISTENING_ADDRESS")
            .expect("No environment variable set `SCHEDULER_LISTENING_ADDRESS`.")
            .to_str()
            .map(|s| s.to_owned())
            .expect("Environment variable resolution failure.");

        let security_conf = SchedulerSecurityConf::default();

        SchedulerMetaInfo {
            name,
            domain,
            listening_address,
            security_conf,
        }
    }
}
