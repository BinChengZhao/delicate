use crate::prelude::*;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Display)]
#[display(fmt = "time:{}", time)]

pub struct HealthScreenUnit {
    pub time: u64,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, Display)]
#[display(fmt = "health-screen-unit:{} ", health_screen_unit)]

pub struct SignedHealthScreenUnit {
    pub health_screen_unit: HealthScreenUnit,
    #[serde(with = "hex")]
    pub signature: Vec<u8>,
}

impl Default for HealthScreenUnit {
    fn default() -> Self {
        let time = get_timestamp();
        HealthScreenUnit { time }
    }
}

impl HealthScreenUnit {
    pub fn set_time(mut self, time: u64) -> Self {
        self.time = time;
        self
    }

    pub fn sign(
        self,
        token: Option<&str>,
    ) -> Result<SignedHealthScreenUnit, crate::error::CommonError> {
        let signature = make_signature(&self, token)?;
        Ok(SignedHealthScreenUnit {
            health_screen_unit: self,
            signature,
        })
    }
}

impl SignedHealthScreenUnit {
    pub fn verify(&self, token: Option<&str>) -> Result<(), crate::error::CommonError> {
        let SignedHealthScreenUnit {
            ref health_screen_unit,
            ref signature,
        } = self;

        verify_signature_by_raw_data(health_screen_unit, token, signature)
    }

    pub fn get_health_screen_unit_after_verify(
        self,
        token: Option<&str>,
    ) -> Result<HealthScreenUnit, crate::error::CommonError> {
        self.verify(token)?;
        let SignedHealthScreenUnit {
            health_screen_unit, ..
        } = self;

        Ok(health_screen_unit)
    }
}
