-- Your SQL goes here

CREATE TABLE casbin_rule (
`id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT 'Self-incrementing id',
`ptype` varchar(64) NOT NULL DEFAULT '' COMMENT 'ptype for casbin',
`v0` varchar(64) NOT NULL DEFAULT '' COMMENT 'Dynamic fields for casbin, adapted to the model',
`v1` varchar(64) NOT NULL DEFAULT '' COMMENT 'Dynamic fields for casbin, adapted to the model',
`v2` varchar(64) NOT NULL DEFAULT '' COMMENT 'Dynamic fields for casbin, adapted to the model',
`v3` varchar(64) NOT NULL DEFAULT '' COMMENT 'Dynamic fields for casbin, adapted to the model',
`v4` varchar(64) NOT NULL DEFAULT '' COMMENT 'Dynamic fields for casbin, adapted to the model',
`v5` varchar(64) NOT NULL DEFAULT '' COMMENT 'Dynamic fields for casbin, adapted to the model',
PRIMARY KEY (`id`),
UNIQUE KEY `casbin_unique_key_idx` (`ptype`, `v0`, `v1`, `v2`, `v3`, `v4`, `v5`)
)ENGINE INNODB DEFAULT CHARSET=utf8mb4 COMMENT 'Casbin Policy Table';

INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'developer', 'task', 'create');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'developer', 'task', 'list');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'developer', 'task', 'update');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'developer', 'task', 'run');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'task_admin', 'task', 'advance');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'task_admin', 'task', 'list');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'task_admin', 'task', 'update');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'task_admin', 'task', 'delete');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'task_admin', 'task', 'run');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'task_admin', 'task', 'create');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'task_admin', 'task', 'suspend');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'task_admin', 'task_instance', 'kill');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'task_admin', 'task_log', 'delete');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'task_admin', 'task_log', 'list');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'task_admin', 'task_log', 'detail');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'processor_admin', 'executor_processor', 'activate');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'processor_admin', 'executor_processor', 'update');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'processor_admin', 'executor_processor', 'delete');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'processor_admin', 'executor_processor', 'list');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'processor_admin', 'executor_processor', 'create');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'group_admin', 'executor_group', 'update');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'group_admin', 'executor_group', 'delete');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'group_admin', 'executor_group', 'list');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'group_admin', 'executor_group', 'create');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'group_admin', 'executor_group', 'detail');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'group_admin', 'executor_processor_bind', 'create');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'group_admin', 'executor_processor_bind', 'update');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'group_admin', 'executor_processor_bind', 'list');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'group_admin', 'executor_processor_bind', 'delete');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'user_admin', 'user', 'delete');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'user_admin', 'user', 'update');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'user_admin', 'user', 'create');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'user_admin', 'user', 'list');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'user_admin', 'user', 'append_role');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'user_admin', 'user', 'delete_role');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'user_admin', 'user', 'append_permission');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'user_admin', 'user', 'delete_permission');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'user_admin', 'user', 'roles');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'user_admin', 'user', 'permissions');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'user_admin', 'permission', 'list');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'user_admin', 'role', 'list');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'user_admin', 'role', 'permission_detail');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'user_admin', 'role', 'users');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'log_admin', 'user_login_log', 'list');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'log_admin', 'operation_log', 'list');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`, `v2`) VALUES ('p', 'log_admin', 'operation_log', 'detail');

INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`) VALUES ('g', 'team_leader', 'task_admin');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`) VALUES ('g', 'team_leader', 'processor_admin');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`) VALUES ('g', 'team_leader', 'group_admin');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`) VALUES ('g', 'team_leader', 'user_admin');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`) VALUES ('g', 'team_leader', 'log_admin');

INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`) VALUES ('g', 'developer', 'processor_admin');
INSERT INTO `casbin_rule` (`ptype`, `v0`, `v1`) VALUES ('g', 'developer', 'log_admin');