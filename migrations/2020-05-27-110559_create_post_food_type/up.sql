CREATE TABLE post_food_type
(
    `postId`     INT(11) NOT NULL,
    `foodTypeId` INT(11) NOT NULL,
    PRIMARY KEY (`postId`, `foodTypeId`),
    INDEX `IDX_32954178aad2d257898d82e598` (`postId` ASC) VISIBLE,
    INDEX `IDX_74da3c6ddf988eba0db8ae2ca2` (`foodTypeId` ASC) VISIBLE,
    CONSTRAINT `FK_32954178aad2d257898d82e598d`
        FOREIGN KEY (`postId`)
            REFERENCES post (`id`)
            ON DELETE CASCADE
            ON UPDATE NO ACTION,
    CONSTRAINT `FK_74da3c6ddf988eba0db8ae2ca2b`
        FOREIGN KEY (`foodTypeId`)
            REFERENCES food_type (`id`)
            ON DELETE CASCADE
            ON UPDATE NO ACTION
)
    DEFAULT CHARACTER SET = utf8mb4;
