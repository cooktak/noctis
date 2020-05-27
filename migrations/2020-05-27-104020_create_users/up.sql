CREATE TABLE users (
    `id` INT(11) NOT NULL AUTO_INCREMENT,
    `birthday` DATETIME NOT NULL,
    `createTime` DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    `gender` ENUM('etc', 'female', 'male', 'private') NOT NULL,
    `nickname` VARCHAR(16) NOT NULL,
    `password` VARCHAR(255) NOT NULL,
    `username` VARCHAR(255) NOT NULL,
    `userTag` INT(11) NOT NULL,
    `photoLink` VARCHAR(255) NULL DEFAULT NULL,
    PRIMARY KEY (`id`)
)
AUTO_INCREMENT = 30
DEFAULT CHARACTER SET = utf8mb4;
