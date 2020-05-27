CREATE TABLE post_cookery
(
    `postId`    INT(11) NOT NULL,
    `cookeryId` INT(11) NOT NULL,
    PRIMARY KEY (`postId`, `cookeryId`),
    INDEX `IDX_b5109155d262c0accee8a0a5da` (`postId` ASC) VISIBLE,
    INDEX `IDX_9e089190f0c7f9e7ee586d1c07` (`cookeryId` ASC) VISIBLE,
    CONSTRAINT `FK_9e089190f0c7f9e7ee586d1c076`
        FOREIGN KEY (`cookeryId`)
            REFERENCES cookery (`id`)
            ON DELETE CASCADE
            ON UPDATE NO ACTION,
    CONSTRAINT `FK_b5109155d262c0accee8a0a5da1`
        FOREIGN KEY (`postId`)
            REFERENCES post (`id`)
            ON DELETE CASCADE
            ON UPDATE NO ACTION
)
    DEFAULT CHARACTER SET = utf8mb4;