use crate::error::InitSchedulerError;
use crate::prelude::*;
use service_binding::BindRequest;

pub trait SecurityRsaKey<T: TryFrom<pem::Pem>>
where
    InitSchedulerError: From<<T as std::convert::TryFrom<pem::Pem>>::Error>,
{
    /// Get delicate-executor's security key from env.
    fn get_app_rsa_key(key_name: &str) -> Result<T, InitSchedulerError> {
        let key_path = env::var_os(key_name)
            .ok_or_else(|| InitSchedulerError::MisEnvVar(String::from(key_name)))?;

        let key_pem = fs::read(key_path)?;
        let key: T = pem::parse(key_pem)?.try_into()?;
        Ok(key)
    }
}

impl SecurityRsaKey<RSAPrivateKey> for SecurityeKey<RSAPrivateKey> {}

impl SecurityRsaKey<RSAPublicKey> for SecurityeKey<RSAPublicKey> {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityeKey<T>(pub T);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookieConf {
    pub domain: String,
    pub http_only: bool,
    pub secure: bool,
}

impl Default for CookieConf {
    fn default() -> Self {
        let domain = env::var("SCHEDULER_COOKIE_DOMAIN")
            .expect("Without `SCHEDULER_COOKIE_DOMAIN` set in .env");

        let http_only = true;
        let secure = false;
        Self {
            domain,
            http_only,
            secure,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerSecurityConf {
    pub cookie_conf: CookieConf,
    pub security_level: SecurityLevel,
    pub rsa_private_key: Option<SecurityeKey<RSAPrivateKey>>,
}

impl Default for SchedulerSecurityConf {
    fn default() -> Self {
        let cookie_conf = CookieConf::default();
        let security_level = SecurityLevel::get_app_security_level();
        let rsa_private_key =
            SecurityeKey::<RSAPrivateKey>::get_app_rsa_key("DELICATE_SECURITY_PRIVATE_KEY");

        if matches!(security_level, SecurityLevel::Normal if rsa_private_key.is_err()) {
            error!(
                "{}",
                rsa_private_key
                    .err()
                    .map(|e| "Initialization failed because: ".to_owned() + (e.to_string().as_ref()))
                    .unwrap_or_default()
            );
            unreachable!("When the security level is Normal, the initialization `delicate-scheduler` must contain the secret key (DELICATE_SECURITY_PRIVATE_KEY)");
        }

        Self {
            cookie_conf,
            security_level: SecurityLevel::get_app_security_level(),
            rsa_private_key: rsa_private_key.map(SecurityeKey).ok(),
        }
    }
}

#[derive(Debug)]
pub struct ExecutorSecurityConf {
    pub security_level: SecurityLevel,
    pub rsa_public_key: Option<SecurityeKey<RSAPublicKey>>,
    pub bind_scheduler: BindScheduler,
}

#[derive(Debug)]
pub struct BindScheduler {
    pub inner: RwLock<Option<BindRequest>>,
    pub token: RwLock<Option<String>>,
}

impl ExecutorSecurityConf {
    pub fn generate_token(&self) -> Option<String> {
        match self.security_level {
            SecurityLevel::Normal => Some(repeat_with(fastrand::alphanumeric).take(32).collect()),
            SecurityLevel::ZeroRestriction => None,
        }
    }

    pub async fn get_bind_scheduler_inner_ref(&self) -> RwLockReadGuard<'_, Option<BindRequest>> {
        self.bind_scheduler.inner.read().await
    }

    pub async fn get_bind_scheduler_inner_mut(&self) -> RwLockWriteGuard<'_, Option<BindRequest>> {
        self.bind_scheduler.inner.write().await
    }

    pub async fn get_bind_scheduler_token_ref(&self) -> RwLockReadGuard<'_, Option<String>> {
        self.bind_scheduler.token.read().await
    }

    pub async fn get_bind_scheduler_token_mut(&self) -> RwLockWriteGuard<'_, Option<String>> {
        self.bind_scheduler.token.write().await
    }
}

impl Default for BindScheduler {
    fn default() -> BindScheduler {
        let inner = RwLock::new(None);
        let token = RwLock::new(None);

        BindScheduler { inner, token }
    }
}

impl Default for ExecutorSecurityConf {
    fn default() -> Self {
        let security_level = SecurityLevel::get_app_security_level();
        let rsa_public_key =
            SecurityeKey::<RSAPublicKey>::get_app_rsa_key("DELICATE_SECURITY_PUBLIC_KEY");

        if matches!(security_level, SecurityLevel::Normal if rsa_public_key.is_err()) {
            error!(
                "{}",
                rsa_public_key
                    .err()
                    .map(|e| "Initialization failed because: ".to_owned() + (e.to_string().as_ref()))
                    .unwrap_or_default()
            );
            unreachable!("When the security level is Normal, the initialization `delicate-executor` must contain the secret key (DELICATE_SECURITY_PUBLIC_KEY)");
        }

        let bind_scheduler = BindScheduler::default();

        Self {
            security_level: SecurityLevel::get_app_security_level(),
            rsa_public_key: rsa_public_key.map(SecurityeKey).ok(),
            bind_scheduler,
        }
    }
}

impl ExecutorSecurityConf {
    pub fn get_rsa_public_key(&self) -> Option<&RSAPublicKey> {
        self.rsa_public_key.as_ref().map(|k| &k.0)
    }
}

/// Delicate's security level.
/// The distinction in security level is reflected at `bind_executor-api`.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// There are no strict restrictions.
    ZeroRestriction,
    /// Normal security validation, encrypted validation is required at `bind_executor-api`.
    Normal,
}

impl SecurityLevel {
    /// Get delicate-scheduler's security level from env.
    pub fn get_app_security_level() -> Self {
        env::var_os("DELICATE_SECURITY_LEVEL").map_or(SecurityLevel::default(), |e| {
            e.to_str()
                .and_then(|s| u16::from_str(s).ok())
                .and_then(|e| e.try_into().ok())
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

pub fn make_signature<T: Serialize>(
    data: &T,
    token: Option<&str>,
) -> Result<Vec<u8>, crate::error::CommonError> {
    match token {
        Some(token) if !token.is_empty() => {
            let json_str = to_json_string(data)?;
            let raw_str = json_str + token;
            let sign = digest(&SHA256, raw_str.as_bytes()).as_ref().to_vec();
            Ok(sign)
        }
        _ => Ok(Vec::default()),
    }
}

pub fn verify_signature_by_raw_data<T: Serialize>(
    data: &T,
    token: Option<&str>,
    signature: &[u8],
) -> Result<(), crate::error::CommonError> {
    let signature_new = make_signature(data, token)?;
    if signature_new.eq(signature) {
        Ok(())
    } else {
        Err(crate::error::CommonError::DisVerify)
    }
}

#[test]
fn test_rsa_crypt() {
    use crate::prelude::*;
    use rand::rngs::OsRng;
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
    use crate::prelude::*;
    use rand::rngs::OsRng;
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
