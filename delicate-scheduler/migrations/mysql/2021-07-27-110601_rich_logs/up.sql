CREATE TABLE operation_log (
`id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT 'Self-incrementing id',
`name` varchar(64) NOT NULL DEFAULT '' COMMENT 'Operation module name',
`table_id` bigint(20) unsigned NOT NULL DEFAULT '0' COMMENT 'Operation table id',
`operation_type` tinyint(4) NOT NULL DEFAULT '1' COMMENT 'Operation type: 1 add 2 modify 3 delete',
`user_id` bigint(20) unsigned NOT NULL DEFAULT '0' COMMENT 'Operation user id',
`user_name` varchar(64) NOT NULL DEFAULT '' COMMENT 'Operation user name',
`operation_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Operation time',
PRIMARY KEY (`id`),
KEY `idx_table_id` (`table_id`) USING BTREE,
KEY `idx_operation_time` (`operation_time`) USING BTREE,
KEY `idx_user_id_type_time` (`user_id`,`operation_type`,`operation_time`) USING BTREE
)ENGINE INNODB DEFAULT CHARSET=utf8mb4 COMMENT 'User Operation Log Record Table';


CREATE TABLE operation_log_detail (
`id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT 'Self-incrementing id',
`operation_log_id` bigint(20) unsigned NOT NULL DEFAULT '0' COMMENT 'Operation log id',
`column_comment` text NOT NULL COMMENT 'Field Description',
`values` text NOT NULL COMMENT 'Values',
PRIMARY KEY (`id`),
KEY `idx_operation_log_id` (`operation_log_id`) USING BTREE
)ENGINE INNODB DEFAULT CHARSET=utf8mb4 COMMENT 'Operation log details table';


 ALTER TABLE `user_login_log` ADD `user_name` varchar(64) NOT NULL DEFAULT '' COMMENT 'Login user name' ;

