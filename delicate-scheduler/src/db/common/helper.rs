use super::prelude::*;
use concat_idents::concat_idents;
use db::model::*;
use state::operation_log::OperationType;
use std::string::ToString;

pub(crate) type NewOperationLogPair = (NewOperationLog, NewOperationLogDetail);
pub(crate) type NewOperationLogPairOption = Option<NewOperationLogPair>;

// TDD.
// operates!("task" , 1 , )
// operates!( name, session(user-id, user-name) , operation_type, values, column_comment);
// operates!( name, session , operation_type, values);

pub trait SeekTableId {
    fn seek_table_id(&self) -> u64 {
        0
    }
}

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize)]
pub struct CommonTableRecord {
    pub id: i64,
    pub description: &'static str,
}

impl CommonTableRecord {
    pub fn set_id(mut self, id: i64) -> Self {
        self.id = id;
        self
    }

    pub fn set_description(mut self, description: &'static str) -> Self {
        self.description = description;
        self
    }
}

macro_rules! impl_seek_table_id_unify{
    ($($target:ty),+) => {
        $(impl SeekTableId for $target {

            fn seek_table_id(&self) -> u64
            {
                self.id as u64
            }

        }

        impl SeekTableId for &$target {

            fn seek_table_id(&self) -> u64
            {
                self.id as u64
            }

        }

        )+
    };

    ($($target:ty => $id:expr),+) => {
        $(impl SeekTableId for $target {

            fn seek_table_id(&self) -> u64
            {
                $id
            }

        }

        impl SeekTableId for &$target {

            fn seek_table_id(&self) -> u64
            {
                $id
            }

        }

        )+
    }
}

impl_seek_table_id_unify!(
    CommonTableRecord,
    TaskLog,
    TaskLogExtend,
    task::Task,
    UpdateTask,
    User,
    UpdateUser,
    TaskBind,
    ExecutorProcessor,
    UpdateExecutorProcessor,
    ExecutorProcessorBind,
    UpdateExecutorProcessorBind,
    ExecutorGroup,
    UpdateExecutorGroup
);
impl_seek_table_id_unify!(NewTaskLog=>0, NewTask=>0, NewUser=>0, NewTaskBind=>0, NewExecutorProcessor=>0, NewExecutorProcessorBind=>0, NewExecutorGroup=>0, NewExecutorProcessorBinds=>0);

pub(crate) fn generate_operation_log(
    operation_name: impl ToString,
    session: &Session,
    operation_type: OperationType,
    value: impl Serialize + SeekTableId,
    column_comment: impl Serialize,
) -> Result<(NewOperationLog, NewOperationLogDetail), CommonError> {
    let name = operation_name.to_string();
    let table_id = value.seek_table_id();
    let operation_type = operation_type as i8;
    let user_id = session.get::<u64>("user_id")?.unwrap_or_default();
    let user_name = session.get::<String>("user_name")?.unwrap_or_default();
    let operation_log_id = 0;
    let column_comment = to_json_string(&column_comment)?;
    let values = to_json_string(&value)?;

    let new_operation_log = NewOperationLog {
        name,
        table_id,
        operation_type,
        user_id,
        user_name,
    };
    let new_operation_log_detail = NewOperationLogDetail {
        operation_log_id,
        column_comment,
        values,
    };

    Ok((new_operation_log, new_operation_log_detail))
}

pub(crate) fn generate_operation_addtion_log(
    operation_name: impl ToString,
    session: &Session,
    values: impl Serialize + SeekTableId,
    column_comment: impl Serialize,
) -> Result<(NewOperationLog, NewOperationLogDetail), CommonError> {
    generate_operation_log(
        operation_name,
        session,
        OperationType::Addition,
        values,
        column_comment,
    )
}

pub(crate) fn generate_operation_modify_log(
    operation_name: impl ToString,
    session: &Session,
    values: impl Serialize + SeekTableId,
    column_comment: impl Serialize,
) -> Result<(NewOperationLog, NewOperationLogDetail), CommonError> {
    generate_operation_log(
        operation_name,
        session,
        OperationType::Modify,
        values,
        column_comment,
    )
}

pub(crate) fn generate_operation_delete_log(
    operation_name: impl ToString,
    session: &Session,
    values: impl Serialize + SeekTableId,
    column_comment: impl Serialize,
) -> Result<(NewOperationLog, NewOperationLogDetail), CommonError> {
    generate_operation_log(
        operation_name,
        session,
        OperationType::Delete,
        values,
        column_comment,
    )
}

macro_rules! generate_operation_log_fn{
    ($(($operation_name:expr => $column_comment:expr)),+) => {
       $(
            concat_idents!(fn_name = generate_operation_, $operation_name, _, "addtion", _log {
               #[allow(dead_code)]
               pub(crate) fn fn_name(session: &Session, values: impl Serialize + SeekTableId)
               -> Result<(NewOperationLog, NewOperationLogDetail), CommonError> {
                   generate_operation_addtion_log($operation_name, session, values, $column_comment)
               }
            });

            concat_idents!(fn_name = generate_operation_, $operation_name, _, "modify", _log {
                #[allow(dead_code)]
                pub(crate) fn fn_name(session: &Session, values: impl Serialize + SeekTableId)
                -> Result<(NewOperationLog, NewOperationLogDetail), CommonError> {
                    generate_operation_modify_log($operation_name, session, values, $column_comment)
                }
             });

             concat_idents!(fn_name = generate_operation_, $operation_name, _, "delete", _log {
                #[allow(dead_code)]
                pub(crate) fn fn_name(session: &Session, values: impl Serialize + SeekTableId)
                -> Result<(NewOperationLog, NewOperationLogDetail), CommonError> {
                    generate_operation_delete_log($operation_name, session, values, $column_comment)
                }
             });

    )+

    }
}

// TODO: `column_comment` can generated by const fn.
generate_operation_log_fn!(("task"=>""), ("task_log"=>""), ("executor_processor"=>""), ("executor_group"=>""), ("executor_processor_bind"=>""), ("user"=>"") );
