use super::prelude::*;
use db::model::*;
use db::schema::operation_log;
use state::operation_log::OperationType;
use std::string::ToString;

// TDD.
// operates!("task" , 1 , )
// operates!( name, session(user-id, user-name) , operation_type, values, column_comment);
// operates!( name, session , operation_type, values);

// `column_comment` can generated by const fn.

pub trait SeekTableId {
    fn seek_table_id(&self) -> i64 {
        0
    }
}

macro_rules! impl_seek_table_id_unify{
    ($($target:ty),+) => {
        $(impl SeekTableId for $target {

            fn seek_table_id(&self) -> i64
            {
                self.id
            }

        })+
    }
}

impl_seek_table_id_unify!(NewTaskLog);

#[allow(dead_code)]
pub(crate) fn operate_log(
    conn: &db::PoolConnection,
    operation_record: NewOperationLog,
) -> Result<(), CommonError> {
    diesel::insert_into(operation_log::table)
        .values(&operation_record)
        .execute(conn)?;
    Ok(())
}

#[allow(dead_code)]

pub(crate) fn generate_operation_log(
    operation_name: impl ToString,
    session: &Session,
    operation_type: OperationType,
    value: impl Serialize + SeekTableId,
    _column_comment: impl Serialize,
) -> (NewOperationLog, NewOperationLogDetail) {
    // name: String,
    // table_id: u64,
    // operation_type: i8,
    // user_id: u64,
    // user_name: String,
    let _name = operation_name.to_string();
    let _table_id = value.seek_table_id();
    let _operation_type = operation_type as i8;
    let _user_id = session.get::<u64>("user_id");
    let _user_name = session.get::<u64>("user_name");

    todo!();
}

#[allow(dead_code)]

pub(crate) fn generate_operation_addtion_log(
    _operation_name: impl ToString,
    _session: &Session,
    _values: impl Serialize,
    _column_comment: impl Serialize,
) -> NewOperationLog {
    todo!();
}

#[allow(dead_code)]

pub(crate) fn generate_operation_modify_log(
    _operation_name: impl ToString,
    _session: &Session,
    _values: impl Serialize,
    _column_comment: impl Serialize,
) -> NewOperationLog {
    todo!();
}

#[allow(dead_code)]

pub(crate) fn generate_operation_delete_log(
    _operation_name: impl ToString,
    _session: &Session,
    _values: impl Serialize,
    _column_comment: impl Serialize,
) -> NewOperationLog {
    todo!();
}

#[allow(dead_code)]

pub(crate) fn generate_operation_task_addtion_log(
    session: &Session,
    values: impl Serialize,
) -> NewOperationLog {
    #[allow(dead_code)]
    struct ColumnCommentUnit {
        title: String,
        description: String,
    }

    generate_operation_addtion_log("task", session, values, "")
}
