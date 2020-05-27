CREATE TABLE post_content
(
    `id`        INT(11)      NOT NULL AUTO_INCREMENT,
    `content`   VARCHAR(255) NOT NULL,
    `photoLink` VARCHAR(255) NOT NULL,
    `postId`    INT(11)      NOT NULL,
    PRIMARY KEY (`id`),
    INDEX `FK_d5e71a3e4c6277474e6af05b8b2` (`postId` ASC) VISIBLE,
    CONSTRAINT `FK_d5e71a3e4c6277474e6af05b8b2`
        FOREIGN KEY (`postId`)
            REFERENCES post (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    DEFAULT CHARACTER SET = utf8mb4;