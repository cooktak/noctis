CREATE TABLE product
(
    `id`                     VARCHAR(255) NOT NULL,
    `info`                   VARCHAR(255) NOT NULL,
    `KANProductCategoryCode` VARCHAR(255) NOT NULL,
    `name`                   VARCHAR(255) NOT NULL,
    `photoLink`              VARCHAR(255) NULL DEFAULT NULL,
    `unit`                   VARCHAR(255) NOT NULL,
    `wight`                  VARCHAR(255) NOT NULL,
    `sellerId`               INT(11)      NOT NULL,
    `vendorId`               INT(11)      NOT NULL,
    PRIMARY KEY (`id`),
    INDEX `FK_d5cac481d22dacaf4d53f900a3f` (`sellerId` ASC) VISIBLE,
    INDEX `FK_921582066aa70b502e78ea92012` (`vendorId` ASC) VISIBLE,
    CONSTRAINT `FK_921582066aa70b502e78ea92012`
        FOREIGN KEY (`vendorId`)
            REFERENCES vendor (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `FK_d5cac481d22dacaf4d53f900a3f`
        FOREIGN KEY (`sellerId`)
            REFERENCES seller (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    DEFAULT CHARACTER SET = utf8mb4;