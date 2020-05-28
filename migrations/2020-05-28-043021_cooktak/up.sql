SET @OLD_UNIQUE_CHECKS = @@UNIQUE_CHECKS, UNIQUE_CHECKS = 0;
SET @OLD_FOREIGN_KEY_CHECKS = @@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS = 0;
SET @OLD_SQL_MODE = @@SQL_MODE, SQL_MODE =
        'ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

-- -----------------------------------------------------
-- Schema cooktak
-- -----------------------------------------------------
CREATE SCHEMA IF NOT EXISTS `cooktak` DEFAULT CHARACTER SET utf8mb4;
USE `cooktak`;

-- -----------------------------------------------------
-- Table `cooktak`.`cookery`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `cooktak`.`cookery`
(
    `id`   INT(11)      NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(255) NOT NULL,
    PRIMARY KEY (`id`)
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table `cooktak`.`food_type`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `cooktak`.`food_type`
(
    `id`   INT(11)      NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(255) NOT NULL,
    PRIMARY KEY (`id`)
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table `cooktak`.`ingredient`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `cooktak`.`ingredient`
(
    `id`       INT(11)      NOT NULL AUTO_INCREMENT,
    `name`     VARCHAR(255) NOT NULL,
    `parentId` INT(11)      NULL DEFAULT NULL,
    PRIMARY KEY (`id`),
    INDEX `FK_9c1af359bf6af1aebeea653b0ed` (`parentId` ASC),
    CONSTRAINT `FK_9c1af359bf6af1aebeea653b0ed`
        FOREIGN KEY (`parentId`)
            REFERENCES `cooktak`.`ingredient` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB
    AUTO_INCREMENT = 3
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table `cooktak`.`user`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `cooktak`.`user`
(
    `id`         INT(11)                                   NOT NULL AUTO_INCREMENT,
    `birthday`   DATETIME                                  NOT NULL,
    `createTime` DATETIME(6)                               NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    `gender`     ENUM ('etc', 'female', 'male', 'private') NOT NULL,
    `nickname`   VARCHAR(16)                               NOT NULL,
    `password`   VARCHAR(255)                              NOT NULL,
    `username`   VARCHAR(255)                              NOT NULL,
    `userTag`    INT(11)                                   NOT NULL,
    `photoLink`  VARCHAR(255)                              NULL     DEFAULT NULL,
    PRIMARY KEY (`id`)
)
    ENGINE = InnoDB
    AUTO_INCREMENT = 30
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table `cooktak`.`post`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `cooktak`.`post`
(
    `id`         INT(11)      NOT NULL AUTO_INCREMENT,
    `createTime` DATETIME(6)  NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    `title`      VARCHAR(255) NOT NULL,
    `userId`     INT(11)      NOT NULL,
    PRIMARY KEY (`id`),
    INDEX `FK_5c1cf55c308037b5aca1038a131` (`userId` ASC),
    CONSTRAINT `FK_5c1cf55c308037b5aca1038a131`
        FOREIGN KEY (`userId`)
            REFERENCES `cooktak`.`user` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table `cooktak`.`post_content`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `cooktak`.`post_content`
(
    `id`        INT(11)      NOT NULL AUTO_INCREMENT,
    `content`   VARCHAR(255) NOT NULL,
    `photoLink` VARCHAR(255) NOT NULL,
    `postId`    INT(11)      NOT NULL,
    PRIMARY KEY (`id`),
    INDEX `FK_d5e71a3e4c6277474e6af05b8b2` (`postId` ASC),
    CONSTRAINT `FK_d5e71a3e4c6277474e6af05b8b2`
        FOREIGN KEY (`postId`)
            REFERENCES `cooktak`.`post` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table `cooktak`.`post_cookery`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `cooktak`.`post_cookery`
(
    `postId`    INT(11) NOT NULL,
    `cookeryId` INT(11) NOT NULL,
    PRIMARY KEY (`postId`, `cookeryId`),
    INDEX `IDX_b5109155d262c0accee8a0a5da` (`postId` ASC),
    INDEX `IDX_9e089190f0c7f9e7ee586d1c07` (`cookeryId` ASC),
    CONSTRAINT `FK_9e089190f0c7f9e7ee586d1c076`
        FOREIGN KEY (`cookeryId`)
            REFERENCES `cooktak`.`cookery` (`id`)
            ON DELETE CASCADE
            ON UPDATE NO ACTION,
    CONSTRAINT `FK_b5109155d262c0accee8a0a5da1`
        FOREIGN KEY (`postId`)
            REFERENCES `cooktak`.`post` (`id`)
            ON DELETE CASCADE
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table `cooktak`.`post_food_type`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `cooktak`.`post_food_type`
(
    `postId`     INT(11) NOT NULL,
    `foodTypeId` INT(11) NOT NULL,
    PRIMARY KEY (`postId`, `foodTypeId`),
    INDEX `IDX_32954178aad2d257898d82e598` (`postId` ASC),
    INDEX `IDX_74da3c6ddf988eba0db8ae2ca2` (`foodTypeId` ASC),
    CONSTRAINT `FK_32954178aad2d257898d82e598d`
        FOREIGN KEY (`postId`)
            REFERENCES `cooktak`.`post` (`id`)
            ON DELETE CASCADE
            ON UPDATE NO ACTION,
    CONSTRAINT `FK_74da3c6ddf988eba0db8ae2ca2b`
        FOREIGN KEY (`foodTypeId`)
            REFERENCES `cooktak`.`food_type` (`id`)
            ON DELETE CASCADE
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table `cooktak`.`post_ingredient`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `cooktak`.`post_ingredient`
(
    `postId`       INT(11)      NOT NULL,
    `ingredientId` INT(11)      NOT NULL,
    `amount`       INT(11)      NOT NULL,
    `unit`         VARCHAR(255) NOT NULL,
    PRIMARY KEY (`postId`, `ingredientId`),
    INDEX `FK_70e062685b27fccd2dac931581a` (`ingredientId` ASC),
    CONSTRAINT `FK_70e062685b27fccd2dac931581a`
        FOREIGN KEY (`ingredientId`)
            REFERENCES `cooktak`.`ingredient` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `FK_b9f502208ebb1947c1da2c4975e`
        FOREIGN KEY (`postId`)
            REFERENCES `cooktak`.`post` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table `cooktak`.`vendor`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `cooktak`.`vendor`
(
    `id`   INT(11)      NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(255) NOT NULL,
    PRIMARY KEY (`id`)
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table `cooktak`.`seller`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `cooktak`.`seller`
(
    `id`   INT(11)      NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(255) NOT NULL,
    PRIMARY KEY (`id`)
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table `cooktak`.`product`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `cooktak`.`product`
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
    INDEX `FK_d5cac481d22dacaf4d53f900a3f` (`sellerId` ASC),
    INDEX `FK_921582066aa70b502e78ea92012` (`vendorId` ASC),
    CONSTRAINT `FK_921582066aa70b502e78ea92012`
        FOREIGN KEY (`vendorId`)
            REFERENCES `cooktak`.`vendor` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `FK_d5cac481d22dacaf4d53f900a3f`
        FOREIGN KEY (`sellerId`)
            REFERENCES `cooktak`.`seller` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table `cooktak`.`post_product`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `cooktak`.`post_product`
(
    `postId`    INT(11)      NOT NULL,
    `productId` VARCHAR(255) NOT NULL,
    PRIMARY KEY (`postId`, `productId`),
    INDEX `IDX_72e185a9d26cfa23c3648ba854` (`postId` ASC),
    INDEX `IDX_1faaa6e24261b54d14e1067844` (`productId` ASC),
    CONSTRAINT `FK_1faaa6e24261b54d14e10678449`
        FOREIGN KEY (`productId`)
            REFERENCES `cooktak`.`product` (`id`)
            ON DELETE CASCADE
            ON UPDATE NO ACTION,
    CONSTRAINT `FK_72e185a9d26cfa23c3648ba8545`
        FOREIGN KEY (`postId`)
            REFERENCES `cooktak`.`post` (`id`)
            ON DELETE CASCADE
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table `cooktak`.`token`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `cooktak`.`token`
(
    `id`           INT(11)      NOT NULL AUTO_INCREMENT,
    `accessToken`  VARCHAR(255) NOT NULL,
    `refreshToken` VARCHAR(255) NOT NULL,
    `userId`       INT(11)      NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE INDEX `REL_94f168faad896c0786646fa3d4` (`userId` ASC),
    CONSTRAINT `FK_94f168faad896c0786646fa3d4a`
        FOREIGN KEY (`userId`)
            REFERENCES `cooktak`.`user` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB
    AUTO_INCREMENT = 30
    DEFAULT CHARACTER SET = utf8mb4;


SET SQL_MODE = @OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS = @OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS = @OLD_UNIQUE_CHECKS;
