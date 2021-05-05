use super::schema::{task, task::dsl::*};
use super::PoolMysqlConnection;
use diesel::result::Error as DieselError;

#[derive(Queryable, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Task {
    id: i64,
    name: String,
    description: String,
    command: String,
    frequency: String,
    cron_expression: String,
    timeout: i16,
    retry_times: i16,
    retry_interval: i16,
    maximun_parallel_runable_num: i16,
    tag: String,
    status: i16,
}

#[derive(Insertable, Identifiable, AsChangeset, Debug, Default, Serialize, Deserialize)]
#[table_name = "task"]
pub struct NewTask {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) command: String,
    pub(crate) frequency: String,
    pub(crate) cron_expression: String,
    pub(crate) timeout: i16,
    pub(crate) retry_times: i16,
    pub(crate) retry_interval: i16,
    pub(crate) maximun_parallel_runable_num: i16,
    pub(crate) tag: String,
    pub(crate) status: i16,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QueryParamsTask {
    id: Option<i64>,
    name: Option<String>,
    description: Option<String>,
    command: Option<String>,
    frequency: Option<String>,
    cron_expression: Option<String>,
    tag: Option<String>,
    status: Option<i16>,
}

impl QueryParamsTask {
    pub(crate) fn query(self, conn: &PoolMysqlConnection) -> Result<Vec<Task>, DieselError> {
        use diesel::prelude::*;

        let mut statement_builder = task::table
            .into_boxed()
            .select((
                task::id,
                task::name,
                task::description,
                task::command,
                task::frequency,
                task::cron_expression,
                task::timeout,
                task::retry_times,
                task::retry_interval,
                task::maximun_parallel_runable_num,
                task::tag,
                task::status,
            ))
            .filter(task::status.ne(2));

        if let Some(task_id) = self.id {
            statement_builder = statement_builder.filter(id.eq(task_id));
        }

        if let Some(task_status) = self.status {
            statement_builder = statement_builder.filter(status.eq(task_status));
        } else {
            statement_builder = statement_builder.filter(status.ne(2));
        }

        if let Some(task_name) = self.name {
            statement_builder = statement_builder.filter(name.eq(task_name));
        }

        if let Some(task_description) = self.description {
            statement_builder = statement_builder.filter(description.eq(task_description));
        }

        if let Some(task_command) = self.command {
            statement_builder = statement_builder.filter(command.eq(task_command));
        }

        if let Some(task_frequency) = self.frequency {
            statement_builder = statement_builder.filter(frequency.eq(task_frequency));
        }

        if let Some(task_cron_expression) = self.cron_expression {
            statement_builder = statement_builder.filter(cron_expression.eq(task_cron_expression));
        }

        if let Some(task_tag) = self.tag {
            statement_builder = statement_builder.filter(tag.eq(task_tag));
        }

        statement_builder.order(id.desc()).load::<Task>(conn)
    }
}
