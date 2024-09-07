-- Modify "comments" table
ALTER TABLE `comments` DROP INDEX `user_name_fk`, DROP FOREIGN KEY `user_name_fk`;
