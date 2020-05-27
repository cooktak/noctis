CREATE TABLE post
(
    `id`         INT(11)      NOT NULL AUTO_INCREMENT,
    `createTime` DATETIME(6)  NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    `title`      VARCHAR(255) NOT NULL,
    `userId`     INT(11)      NOT NULL,
    PRIMARY KEY (`id`),
    INDEX `FK_5c1cf55c308037b5aca1038a131` (`userId` ASC) VISIBLE,
    CONSTRAINT `FK_5c1cf55c308037b5aca1038a131`
        FOREIGN KEY (`userId`)
            REFERENCES user (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    DEFAULT CHARACTER SET = utf8mb4;