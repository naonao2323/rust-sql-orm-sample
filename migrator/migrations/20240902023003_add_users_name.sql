-- Modify "users" table
ALTER TABLE `users` ADD COLUMN `aaa` int NOT NULL, DROP INDEX `manager_fk`, DROP FOREIGN KEY `manager_fk`;
