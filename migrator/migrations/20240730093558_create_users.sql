-- Create "users" table
CREATE TABLE `users` (`id` int NOT NULL, `name` varchar(255) NOT NULL, `manager_id` int NOT NULL, PRIMARY KEY (`id`), INDEX `manager_fk` (`manager_id`), CONSTRAINT `manager_fk` FOREIGN KEY (`manager_id`) REFERENCES `users` (`id`) ON UPDATE NO ACTION ON DELETE CASCADE) CHARSET utf8mb4 COLLATE utf8mb4_0900_ai_ci;
