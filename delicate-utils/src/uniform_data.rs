use super::prelude::*;

pub trait UniformData: Debug + Clone + Serialize {}

impl<T: Debug + Clone + Serialize> UniformData for T {}

/// Uniform public message response format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedResponseMessages<T: UniformData> {
    code: i8,
    msg: String,
    data: T,
}

impl<T: UniformData> UnifiedResponseMessages<T> {
    pub fn success_with_data(data: T) -> Self {
        let msg = String::default();
        let code = i8::default();
        UnifiedResponseMessages { code, msg, data }
    }

    #[allow(dead_code)]
    pub fn error_with_data(data: T) -> Self {
        let code = -1;
        let msg = String::default();
        UnifiedResponseMessages { code, msg, data }
    }

    pub fn customized_error_msg(mut self, msg: String) -> Self {
        self.msg = msg;

        self
    }

    #[allow(dead_code)]
    pub fn customized_error_code(mut self, code: i8) -> Self {
        self.code = code;

        self
    }

    #[allow(dead_code)]
    pub fn reverse(mut self) -> Self {
        self.code = -1 - self.code;
        self
    }
}

impl<T: UniformData + Default> UnifiedResponseMessages<T> {
    pub fn error() -> Self {
        let msg = String::default();
        let data = T::default();
        UnifiedResponseMessages {
            code: -1,
            msg,
            data,
        }
    }

    pub fn success() -> Self {
        let msg = String::default();
        let data = T::default();
        UnifiedResponseMessages { code: 0, msg, data }
    }
}

impl<T: UniformData + Default, E: std::error::Error> From<Result<T, E>>
    for UnifiedResponseMessages<T>
{
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(d) => Self::success_with_data(d),
            Err(e) => {
                let message = format!(
                    "{} ({})",
                    e.to_string(),
                    e.source().map(|s| { s.to_string() }).unwrap_or_default()
                );
                Self::error().customized_error_msg(message)
            }
        }
    }
}
