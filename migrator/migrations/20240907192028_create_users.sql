-- Modify "comments" table
ALTER TABLE `comments` ADD COLUMN `user_fk` int NOT NULL, ADD INDEX `user_name_fk` (`user_fk`), ADD CONSTRAINT `user_name_fk` FOREIGN KEY (`user_fk`) REFERENCES `users` (`id`) ON UPDATE NO ACTION ON DELETE CASCADE;
