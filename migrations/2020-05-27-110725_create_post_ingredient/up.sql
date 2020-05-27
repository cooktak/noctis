CREATE TABLE post_ingredient
(
    `postId`       INT(11)      NOT NULL,
    `ingredientId` INT(11)      NOT NULL,
    `amount`       INT(11)      NOT NULL,
    `unit`         VARCHAR(255) NOT NULL,
    PRIMARY KEY (`postId`, `ingredientId`),
    INDEX `FK_70e062685b27fccd2dac931581a` (`ingredientId` ASC) VISIBLE,
    CONSTRAINT `FK_70e062685b27fccd2dac931581a`
        FOREIGN KEY (`ingredientId`)
            REFERENCES ingredient (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `FK_b9f502208ebb1947c1da2c4975e`
        FOREIGN KEY (`postId`)
            REFERENCES post (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    DEFAULT CHARACTER SET = utf8mb4;