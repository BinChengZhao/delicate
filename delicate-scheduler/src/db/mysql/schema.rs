table! {
    executor_group (id) {
        id -> Bigint,
        name -> Varchar,
        description -> Varchar,
        tag -> Varchar,
        status -> Smallint,
        created_time -> Timestamp,
        deleted_time -> Nullable<Timestamp>,
    }
}

table! {
    executor_processor (id) {
        id -> Bigint,
        name -> Varchar,
        host -> Bigint,
        port -> Smallint,
        description -> Varchar,
        tag -> Varchar,
        status -> Smallint,
        created_time -> Timestamp,
        deleted_time -> Nullable<Timestamp>,
    }
}

table! {
    executor_processor_group (id) {
        id -> Bigint,
        name -> Varchar,
        group_id -> Bigint,
        executor_id -> Bigint,
        weight -> Smallint,
        status -> Smallint,
        created_time -> Timestamp,
        deleted_time -> Nullable<Timestamp>,
    }
}

table! {
    posts (id) {
        id -> Bigint,
        title -> Varchar,
        body -> Text,
        published -> Smallint,
    }
}

table! {
    task (id) {
        id -> Bigint,
        name -> Varchar,
        description -> Varchar,
        command -> Varchar,
        frequency -> Varchar,
        cron_expression -> Varchar,
        timeout -> Smallint,
        retry_times -> Smallint,
        retry_interval -> Smallint,
        maximun_parallel_runable_num -> Smallint,
        tag -> Varchar,
        status -> Smallint,
        created_time -> Timestamp,
        deleted_time -> Nullable<Timestamp>,
    }
}

table! {
    task_log (id) {
        id -> Bigint,
        task_id -> Bigint,
        record_id -> Bigint,
        name -> Varchar,
        description -> Varchar,
        command -> Varchar,
        frequency -> Varchar,
        cron_expression -> Varchar,
        maximun_parallel_runable_num -> Smallint,
        tag -> Varchar,
        status -> Smallint,
        created_time -> Timestamp,
        deleted_time -> Nullable<Timestamp>,
        executor_processor_id -> Bigint,
        executor_processor_name -> Varchar,
        executor_processor_host -> Bigint,
    }
}

table! {
    user_auth (id) {
        id -> Bigint,
        user_id -> Unsigned<Bigint>,
        identity_type -> Unsigned<Tinyint>,
        identifier -> Varchar,
        certificate -> Varchar,
        status -> Tinyint,
        created_time -> Timestamp,
        updated_time -> Timestamp,
    }
}

table! {
    user_base (user_id) {
        user_id -> Bigint,
        user_name -> Varchar,
        nick_name -> Varchar,
        mobile -> Varchar,
        email -> Varchar,
        face -> Varchar,
        status -> Tinyint,
        created_time -> Timestamp,
        updated_time -> Timestamp,
    }
}

table! {
    user_info_update (id) {
        id -> Bigint,
        user_id -> Unsigned<Bigint>,
        attribute_name -> Varchar,
        attribute_old_val -> Varchar,
        attribute_new_val -> Varchar,
        updated_time -> Timestamp,
    }
}

table! {
    user_login_log (id) {
        id -> Bigint,
        user_id -> Unsigned<Bigint>,
        login_type -> Unsigned<Tinyint>,
        command -> Unsigned<Tinyint>,
        lastip -> Bigint,
        created_time -> Timestamp,
    }
}

table! {
    user_register_log (id) {
        id -> Bigint,
        user_id -> Unsigned<Bigint>,
        register_method -> Unsigned<Tinyint>,
        register_time -> Timestamp,
        register_ip -> Bigint,
    }
}

allow_tables_to_appear_in_same_query!(
    executor_group,
    executor_processor,
    executor_processor_group,
    posts,
    task,
    task_log,
    user_auth,
    user_base,
    user_info_update,
    user_login_log,
    user_register_log,
);
