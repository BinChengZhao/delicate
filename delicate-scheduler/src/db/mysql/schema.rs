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
        timeout -> SmallInt,
        retry_times -> SmallInt,
        retry_interval -> SmallInt,
        maximun_parallel_runable_num -> SmallInt,
        tag -> Varchar,
        status -> SmallInt,
        created_time -> Timestamp,
        deleted_time -> Timestamp,
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
        maximun_parallel_runable_num -> SmallInt,
        tag -> Varchar,
        status -> SmallInt,
        created_time -> Timestamp,
        deleted_time -> Timestamp,
        executor_processor_id -> Bigint,
        executor_processor_name -> Varchar,
        executor_processor_host -> Bigint,

    }
}


table! {
    executor_processor (id) {
        id -> Bigint,
        name -> Varchar,
        host -> Bigint,
        port -> SmallInt,
        description -> Varchar,
        tag -> Varchar,
        status -> SmallInt,
        created_time -> Timestamp,
        deleted_time -> Timestamp,
    }
}

table! {
    executor_group (id) {
        id -> Bigint,
        name -> Varchar,
        description -> Varchar,
        tag -> Varchar,
        status -> SmallInt,
        created_time -> Timestamp,
        deleted_time -> Timestamp,
    }
}

table! {
    executor_processor_group (id) {
        id -> Bigint,
        name -> Varchar,
        group_id -> Bigint,
        executor_id -> Bigint,
        weight -> SmallInt,
        status -> SmallInt,
        created_time -> Timestamp,
        deleted_time -> Timestamp,
    }
}


table! {
    user_auth (id) {
        id -> Bigint,
        user_id -> Bigint,
        identity_type -> Tinyint,
        identifier -> Varchar,
        certificate -> Varchar,
        status -> SmallInt,
        created_time -> Timestamp,
        deleted_time -> Timestamp,
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
        status -> SmallInt,
        created_time -> Timestamp,
        deleted_time -> Timestamp,
    }
}


table! {
    user_login_log (id) {
        id -> Bigint,
        user_id -> Bigint,
        login_type -> Tinyint,
        command -> Tinyint,
        lastip -> Bigint,
        created_time -> Timestamp,
    }
}


table! {
    user_register_log (id) {
        id -> Bigint,
        user_id -> Bigint,
        register_method -> Tinyint,
        register_time -> Timestamp,
        register_ip -> Bigint,
    }
}

table! {
    user_info_update (id) {
        id -> Bigint,
        user_id -> Bigint,
        attribute_name -> Varchar,
        attribute_old_val -> Varchar,
        attribute_new_val -> Varchar,
        updated_time -> Timestamp,
    }
}
