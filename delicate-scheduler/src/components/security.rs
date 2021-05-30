use crate::prelude::*;
use error::InitSchedulerError;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct BindRequest {
    pub(crate) scheduler_host: String,
    pub(crate) executor_name: String,
    pub(crate) executor_machine_id: i16,
    pub(crate) time: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct SignedBindRequest {
    pub(crate) bind_request: BindRequest,
    pub(crate) signature: Vec<u8>,
}

impl BindRequest {
    pub(crate) fn set_scheduler_host(mut self, scheduler_host: String) -> Self {
        self.scheduler_host = scheduler_host;
        self
    }

    pub(crate) fn set_executor_name(mut self, executor_name: String) -> Self {
        self.executor_name = executor_name;
        self
    }

    pub(crate) fn set_executor_machine_id(mut self, executor_machine_id: i16) -> Self {
        self.executor_machine_id = executor_machine_id;
        self
    }

    pub(crate) fn set_time(mut self, time: u64) -> Self {
        self.time = time;
        self
    }

    // Except here, the rest of the interaction is done using token-based symmetric encryption.
    pub(crate) fn sign(
        self,
        priv_key: &RSAPrivateKey,
    ) -> Result<SignedBindRequest, crate::error::BindExecutorError> {
        let json_str = to_json_string(&self)?;

        let signature =
            priv_key.sign(PaddingScheme::new_pkcs1v15_sign(None), json_str.as_bytes())?;

        Ok(SignedBindRequest {
            bind_request: self,
            signature,
        })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct BindResponse {
    pub(crate) token: String,
    pub(crate) time: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct EncryptedBindResponse {
    pub(crate) bind_response: Vec<u8>,
}

impl EncryptedBindResponse {
    pub(crate) fn decrypt_self(
        self,
        priv_key: &RSAPrivateKey,
    ) -> Result<BindResponse, crate::error::BindExecutorError> {
        // Decrypt
        let padding = PaddingScheme::new_pkcs1v15_encrypt();
        let dec_data = priv_key.decrypt(padding, &self.bind_response)?;
        Ok(json_from_slice(&dec_data)?)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SecurityPrivateKey(RSAPrivateKey);

impl SecurityPrivateKey {
    /// Get delicate-executor's security private-key from env.
    pub(crate) fn get_app_rsa_private_key() -> Result<Self, InitSchedulerError> {
        let key_path = env::var_os("DELICATE_SECURITY_PRIVATE_KEY").ok_or(
            InitSchedulerError::MisEnvVar(String::from("DELICATE_SECURITY_PRIVATE_KEY")),
        )?;

        let key_pem = fs::read(key_path)?;
        let key: RSAPrivateKey = pem::parse(key_pem)?.try_into()?;
        Ok(SecurityPrivateKey(key))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SchedulerSecurityConf {
    pub(crate) security_level: SecurityLevel,
    pub(crate) rsa_private_key: Option<SecurityPrivateKey>,
}

impl Default for SchedulerSecurityConf {
    fn default() -> Self {
        let security_level = SecurityLevel::get_app_security_level();
        let rsa_private_key = SecurityPrivateKey::get_app_rsa_private_key();

        assert!(
            matches!(security_level, SecurityLevel::Normal if rsa_private_key.is_err()), "When the security level is Normal, the initialization `delicate-scheduler` must contain the secret key (DELICATE_SECURITY_PRIVATE_KEY)"
        );

        Self {
            security_level: SecurityLevel::get_app_security_level(),
            rsa_private_key: rsa_private_key.ok(),
        }
    }
}

/// Delicate's security level.
/// The distinction in security level is reflected at `bind_executor-api`.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub(crate) enum SecurityLevel {
    /// There are no strict restrictions.
    ZeroRestriction,
    /// Normal security validation, encrypted validation is required at `bind_executor-api`.
    Normal,
}

impl SecurityLevel {
    /// Get delicate-scheduler's security level from env.
    pub(crate) fn get_app_security_level() -> Self {
        env::var_os("DELICATE_SECURITY_LEVEL").map_or(SecurityLevel::default(), |e| {
            e.to_str()
                .map(|s| u16::from_str(s).ok())
                .flatten()
                .map(|e| e.try_into().ok())
                .flatten()
                .expect("Environment Variables `DELICATE_SECURITY_LEVEL` missed.")
        })
    }
}

impl Default for SecurityLevel {
    fn default() -> Self {
        SecurityLevel::ZeroRestriction
    }
}

impl TryFrom<u16> for SecurityLevel {
    type Error = InitSchedulerError;

    fn try_from(value: u16) -> Result<SecurityLevel, InitSchedulerError> {
        match value {
            0 => Ok(SecurityLevel::ZeroRestriction),
            1 => Ok(SecurityLevel::Normal),
            _ => Err(InitSchedulerError::MisEnvVar(String::from("SecurityLevel"))),
        }
    }
}

#[test]
fn test_rsa_crypt() {
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RSAPublicKey::from(&priv_key);

    // Encrypt
    let data = b"hello world";
    let enc_data = pub_key
        .encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &data[..])
        .expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);

    // Decrypt
    let dec_data = priv_key
        .decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &enc_data)
        .expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);
}

#[test]
fn test_rsa_sign() {
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RSAPublicKey::from(&priv_key);

    let data = b"hello world";

    // Sign
    let sign_data = priv_key
        .sign(PaddingScheme::new_pkcs1v15_sign(None), data)
        .expect("failed to sign");
    assert_ne!(&data[..], &sign_data[..]);

    // Verify
    pub_key
        .verify(PaddingScheme::new_pkcs1v15_sign(None), data, &sign_data[..])
        .expect("failed to Verify");
}
