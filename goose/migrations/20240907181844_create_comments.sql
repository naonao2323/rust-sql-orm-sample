-- +goose Up
CREATE TABLE `comments` (
  `test` varchar(255) NOT NULL,
  `user_id` int NOT NULL,
  `id` int NOT NULL,
  PRIMARY KEY (`id`),
  INDEX `user_id_fk` (`user_id`),
  CONSTRAINT `user_id_fk` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON UPDATE NO ACTION ON DELETE CASCADE
) CHARSET utf8mb4 COLLATE utf8mb4_0900_ai_ci;
-- +goose StatementBegin
SELECT 'up SQL query';
-- +goose StatementEnd

-- +goose Down
DROP TABLE comments;
-- +goose StatementBegin
SELECT 'down SQL query';
-- +goose StatementEnd
