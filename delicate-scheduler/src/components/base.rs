use crate::prelude::*;
use crate::security::SchedulerSecurityConf;

pub(crate) type SharedSchedulerMetaInfo = ShareData<SchedulerMetaInfo>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SchedulerMetaInfo {
    name: String,
    domain: String,
    listening_address: String,
    security_conf: SchedulerSecurityConf,
}

impl SchedulerMetaInfo {
    pub(crate) fn get_app_host_name(&self) -> &String {
        &self.domain
    }

    #[allow(dead_code)]
    pub(crate) fn get_app_security_level(&self) -> SecurityLevel {
        self.security_conf.security_level
    }

    pub(crate) fn get_app_security_key(&self) -> Option<&RSAPrivateKey> {
        self.security_conf.rsa_private_key.as_ref().map(|k| &k.0)
    }
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
