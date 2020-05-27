CREATE TABLE token
(
    `id`           INT(11)      NOT NULL AUTO_INCREMENT,
    `accessToken`  VARCHAR(255) NOT NULL,
    `refreshToken` VARCHAR(255) NOT NULL,
    `userId`       INT(11)      NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE INDEX `REL_94f168faad896c0786646fa3d4` (`userId` ASC) VISIBLE,
    CONSTRAINT `FK_94f168faad896c0786646fa3d4a`
        FOREIGN KEY (`userId`)
            REFERENCES user (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB
    AUTO_INCREMENT = 30
    DEFAULT CHARACTER SET = utf8mb4;