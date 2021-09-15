-- Your SQL goes here

CREATE TABLE casbin_rule (
`id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT 'Self-incrementing id',
`ptype` varchar(12) NOT NULL DEFAULT '' COMMENT 'ptype for casbin',
`v0` varchar(12) NOT NULL DEFAULT '' COMMENT 'Dynamic fields for casbin, adapted to the model',
`v1` varchar(12) NOT NULL DEFAULT '' COMMENT 'Dynamic fields for casbin, adapted to the model',
`v2` varchar(12) NOT NULL DEFAULT '' COMMENT 'Dynamic fields for casbin, adapted to the model',
`v3` varchar(12) NOT NULL DEFAULT '' COMMENT 'Dynamic fields for casbin, adapted to the model',
`v4` varchar(12) NOT NULL DEFAULT '' COMMENT 'Dynamic fields for casbin, adapted to the model',
`v5` varchar(12) NOT NULL DEFAULT '' COMMENT 'Dynamic fields for casbin, adapted to the model',
PRIMARY KEY (`id`),
UNIQUE KEY `casbin_unique_key_idx` (`ptype`, `v0`, `v1`, `v2`, `v3`, `v4`, `v5`)
)ENGINE INNODB DEFAULT CHARSET=utf8mb4 COMMENT 'Casbin Policy Table';