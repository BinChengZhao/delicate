-- This file should undo anything in `up.sql`
DROP TABLE operation_log;
DROP TABLE operation_log_detail;
ALTER TABLE `user_login_log` DROP `user_name`;
