-- This file should undo anything in `up.sql`
ALTER TABLE `task` DROP `schedule_type`;
ALTER TABLE `task` DROP `execute_mode`;

