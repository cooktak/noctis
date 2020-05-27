CREATE TABLE post_product
(
    `postId`    INT(11)      NOT NULL,
    `productId` VARCHAR(255) NOT NULL,
    PRIMARY KEY (`postId`, `productId`),
    INDEX `IDX_72e185a9d26cfa23c3648ba854` (`postId` ASC) VISIBLE,
    INDEX `IDX_1faaa6e24261b54d14e1067844` (`productId` ASC) VISIBLE,
    CONSTRAINT `FK_1faaa6e24261b54d14e10678449`
        FOREIGN KEY (`productId`)
            REFERENCES product (`id`)
            ON DELETE CASCADE
            ON UPDATE NO ACTION,
    CONSTRAINT `FK_72e185a9d26cfa23c3648ba8545`
        FOREIGN KEY (`postId`)
            REFERENCES post (`id`)
            ON DELETE CASCADE
            ON UPDATE NO ACTION
)
    DEFAULT CHARACTER SET = utf8mb4;