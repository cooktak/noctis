CREATE TABLE ingredient
(
    `id`       INT(11)      NOT NULL AUTO_INCREMENT,
    `name`     VARCHAR(255) NOT NULL,
    `parentId` INT(11)      NULL DEFAULT NULL,
    PRIMARY KEY (`id`),
    INDEX `FK_9c1af359bf6af1aebeea653b0ed` (`parentId` ASC) VISIBLE,
    CONSTRAINT `FK_9c1af359bf6af1aebeea653b0ed`
        FOREIGN KEY (`parentId`)
            REFERENCES ingredient (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    AUTO_INCREMENT = 3
    DEFAULT CHARACTER SET = utf8mb4;