use super::prelude::*;
use super::schema::{operation_log, operation_log_detail};

#[derive(Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name = "operation_log"]

pub struct OperationLog {
    id: u64,
    name: String,
    table_id: u64,
    operation_type: i8,
    user_id: u64,
    user_name: String,
    operation_time: NaiveDateTime,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name = "operation_log_detail"]

pub struct OperationLogDetail {
    id: u64,
    operation_log_id: u64,
    column_comment: String,
    values: String,
}



#[derive(Insertable, Debug, Default, Serialize, Deserialize)]
#[table_name = "operation_log"]
pub struct NewOperationLog {
   pub(crate) name: String,
   pub(crate) table_id: u64,
   pub(crate) operation_type: i8,
   pub(crate) user_id: u64,
   pub(crate) user_name: String,
}

#[derive(Insertable, Debug, Default, Serialize, Deserialize)]
#[table_name = "operation_log_detail"]

pub struct NewOperationLogDetail {
   pub(crate) operation_log_id: u64,
   pub(crate) column_comment: String,
   pub(crate) values: String,
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct QueryParamsOperationLog {
    id: Option<u64>,
    name: Option<String>,
    table_id: Option<u64>,
    operation_type: Option<i8>,
    user_id: Option<u64>,
    user_name: Option<String>,
    pub(crate) start_time: Option<i64>,
    pub(crate) end_time: Option<i64>,
    pub(crate) per_page: i64,
    pub(crate) page: i64,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]

pub struct OperationLogId {
    pub(crate) operation_log_id: u64,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]

pub struct OperationLogTableId {
    pub(crate) table_id: u64,
}

pub(crate) struct OperationLogQueryBuilder;
impl OperationLogQueryBuilder {
    pub(crate) fn query_all_columns() -> operation_log::BoxedQuery<'static, Mysql> {
        operation_log::table
            .into_boxed()
            .select(operation_log::all_columns)
    }

    pub(crate) fn query_count(
    ) -> operation_log::BoxedQuery<'static, Mysql, diesel::sql_types::Bigint> {
        operation_log::table.into_boxed().count()
    }
}

impl QueryParamsOperationLog {
    pub(crate) fn query_filter<ST>(
        self,
        mut statement_builder: operation_log::BoxedQuery<'static, Mysql, ST>,
    ) -> operation_log::BoxedQuery<'static, Mysql, ST> {

        if let Some(operation_log_id) = self.id {
            statement_builder = statement_builder.filter(operation_log::id.eq(operation_log_id));
        }

        if let Some(operation_log_name) = self.name {
            statement_builder =
                statement_builder.filter(operation_log::name.like(operation_log_name));
        }

        if let Some(table_id) = self.table_id {
            statement_builder = statement_builder.filter(operation_log::table_id.eq(table_id));
        }

        if let Some(operation_type) = self.operation_type {
            statement_builder = statement_builder.filter(operation_log::operation_type.eq(operation_type));
        }

        if let Some(user_id) = self.user_id {
            statement_builder = statement_builder.filter(operation_log::user_id.eq(user_id));
        }

        if let Some(user_name) = self.user_name {
            statement_builder = statement_builder
                .filter(operation_log::user_name.like(user_name));
        }

        // TODO: Input time-range is a date-string.
        //  Get NaiveDateTime by called `parse_from_str`.
        if let Some(start_time) = self.start_time {
            let end_time = self.end_time.unwrap_or_else(|| start_time + 86400 * 3);

            let start_time = NaiveDateTime::from_timestamp(start_time, 0);
            let end_time = NaiveDateTime::from_timestamp(end_time, 0);

            statement_builder =
                statement_builder.filter(operation_log::operation_time.between(start_time, end_time));
        }

        statement_builder.order(operation_log::id.desc())
    }
}
