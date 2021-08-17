#![allow(unused_imports)]
use crate::prelude::*;

cfg_auth_casbin!(
    lazy_static! {
        pub static ref AUTHER: RwLock<Enforcer> = {
           let e = futures_block_on(Enforcer::new("examples/rbac_with_domains_model.conf", "examples/rbac_with_domains_policy.csv")).expect("Unable to read permission file.");
           RwLock::new(e)
        };
    }


    pub(crate) async fn warm_up_auther(){
        AUTHER.write().await.enable_log(true);
    }
);
