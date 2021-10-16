#[allow(unused_imports)]
pub(crate) use super::components::auth::casbin::casbin_event_consumer::{
    handle_event_for_watcher, launch_casbin_rule_events_consumer,
};
#[allow(unused_imports)]
pub(crate) use super::components::auth::casbin::*;
pub(crate) use super::components::base::SchedulerMetaInfo;
pub(crate) use super::components::health_checker::loop_health_check;
pub(crate) use super::components::helper::*;

pub(crate) use super::components::operation_log_consumer::{
    loop_operate_logs, send_option_operation_log_pair,
};
pub(crate) use super::db;
pub(crate) use super::db::common::helper::*;
pub(crate) use super::db::common::{model as common_model, state, types};
pub(crate) use super::db::extension::*;
pub(crate) use super::db::model;

pub(crate) use common_model::PaginateData;

pub(crate) use delicate_utils::consensus_message::security::{self, SecurityLevel};
pub(crate) use delicate_utils::consensus_message::service_binding;
pub(crate) use delicate_utils::consensus_message::{
    executor_processor as delicate_utils_executor_processor,
    health_check as delicate_utils_health_check, task as delicate_utils_task,
    task_log as delicate_utils_task_log,
};
pub(crate) use delicate_utils::error::{AuthServiceError, CommonError};
pub(crate) use delicate_utils::helper_utils::get_unique_id_string;
pub(crate) use delicate_utils::prelude::*;
pub(crate) use delicate_utils::uniform_data::*;

pub(crate) use std::collections::{HashMap, HashSet};
pub(crate) use std::convert::{AsRef, TryFrom};
pub(crate) use std::env;
pub(crate) use std::fmt::Debug;
pub(crate) use std::future::Future;
pub(crate) use std::iter::Iterator;
pub(crate) use std::ops::Deref;

pub(crate) use std::str::FromStr;
pub(crate) use std::string::ToString;
pub(crate) use std::sync::atomic::{AtomicUsize, Ordering};
pub(crate) use std::sync::Arc;
pub(crate) use std::vec::IntoIter;

pub(crate) use std::time::Duration;
pub(crate) use std::time::SystemTime;

pub(crate) use anyhow::Result as AnyResut;
pub(crate) use async_channel::{Receiver as AsyncReceiver, Sender as AsyncSender};
pub(crate) use async_lock::RwLock;
pub(crate) use cached::proc_macro::cached;
pub(crate) use cached::TimedSizedCache;
pub(crate) use casbin::{
    CoreApi, Enforcer, EventData as CasbinEventData, InternalApi, MgmtApi, RbacApi,
    Watcher as CasbinWatcher,
};

pub(crate) use chrono::{DateTime, Duration as ChronoDuration, Local, NaiveDateTime, Timelike};

pub(crate) use delay_timer::prelude::*;
pub(crate) use diesel::mysql::Mysql;
pub(crate) use diesel::prelude::*;
pub(crate) use diesel::query_builder::{AsQuery, AstPass, Query, QueryFragment};
pub(crate) use diesel::query_dsl::methods::LoadQuery;
pub(crate) use diesel::query_dsl::RunQueryDsl;
pub(crate) use diesel::r2d2::CustomizeConnection;
pub(crate) use diesel::result::Error as DieselError;
pub(crate) use diesel::sql_types;
pub(crate) use dotenv::dotenv;
pub(crate) use flexi_logger::writers::FileLogWriterHandle;
pub(crate) use flexi_logger::{
    writers::FileLogWriter, Age, Cleanup, Criterion, FileSpec, Naming, WriteMode,
};
pub(crate) use futures::future::{join, join3, JoinAll};

pub(crate) use tokio::runtime::Builder;
pub(crate) use tokio::runtime::Runtime;
pub(crate) use tokio::spawn as tokio_spawn;
pub(crate) use tokio::task::spawn_blocking;
pub(crate) use tokio::time::{interval, sleep};
pub(crate) use tokio::time::{timeout as tokio_timeout, Timeout as TokioTimeout};
pub(crate) use tracing::{debug, error, info, span, Instrument, Level};
pub(crate) use tracing_subscriber::FmtSubscriber;

pub(crate) use regex::Regex;
pub(crate) use reqwest::{
    Client as RequestClient, Error as RequestError, Response as RequestResponse,
};
pub(crate) use ring::digest::{digest, SHA256};
pub(crate) use rsa::RSAPrivateKey;
pub(crate) use serde::de::DeserializeOwned;
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use serde_json::{from_str as from_json_str, to_string as to_json_string};
pub(crate) use strum::IntoEnumIterator;
pub(crate) use strum_macros::{AsRefStr, EnumIter, IntoStaticStr, ToString as StrumToString};
pub(crate) use validator::{Validate, ValidationErrors};

pub(crate) use poem::http::Method;
pub(crate) use poem::listener::TcpListener;
pub(crate) use poem::middleware::{AddData, CookieJarManager};

pub(crate) use poem::middleware::Cors;
pub(crate) use poem::web::cookie::{Cookie, CookieJar, CookieKey, SignedCookieJar};
pub(crate) use poem::web::{Data, IntoResponse, Json};
pub(crate) use poem::{
    get, handler, post, Endpoint, EndpointExt, Middleware, Request, Response, Route, Server,
};

pub(crate) type AuthServiceResult<T> = Result<T, AuthServiceError>;

pub(crate) const ROLES: [&str; 7] = [
    "developer",
    "task_admin",
    "processor_admin",
    "group_admin",
    "user_admin",
    "log_admin",
    "team_leader",
];
