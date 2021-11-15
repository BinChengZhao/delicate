-- Your SQL goes here
ALTER TABLE `task` ADD `schedule_type` smallint NOT NULL DEFAULT '1' COMMENT 'task scheduling type (1:Centralized, 2:Weakly centralized)' ;
ALTER TABLE `task` ADD `execute_mode` smallint NOT NULL DEFAULT '1' COMMENT 'Execution modes for centralized: (1:broadcast, 2:polling, 3:random)' ;