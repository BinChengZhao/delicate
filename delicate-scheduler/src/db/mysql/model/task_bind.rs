use super::prelude::*;
use super::schema::{task_bind};

#[derive(Queryable, AsChangeset, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "task_bind"]

pub struct TaskBind {
    id: i64,
    task_id: i64,
    bind_id: i64,
    created_time: NaiveDateTime,
}

#[derive(Insertable, Queryable, AsChangeset, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[table_name = "task_bind"]
pub struct NewTaskBind {
   pub(crate) task_id: i64,
   pub(crate) bind_id: i64,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]

pub struct TaskBindId{
   pub(crate) task_bind_id : i64
}