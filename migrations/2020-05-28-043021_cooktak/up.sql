SET @OLD_UNIQUE_CHECKS = @@UNIQUE_CHECKS, UNIQUE_CHECKS = 0;
SET @OLD_FOREIGN_KEY_CHECKS = @@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS = 0;
SET @OLD_SQL_MODE = @@SQL_MODE, SQL_MODE =
        'ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

-- -----------------------------------------------------
-- Table cookery
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS cookery
(
    id   INT(11)      NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table food_type
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS food_type
(
    id   INT(11)      NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table ingredient
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS ingredient
(
    id        INT(11)      NOT NULL AUTO_INCREMENT,
    name      VARCHAR(255) NOT NULL,
    parent_id INT(11)      NULL DEFAULT NULL,
    PRIMARY KEY (id),
    INDEX `FK_9c1af359bf6af1aebeea653b0ed` (parent_id ASC),
    CONSTRAINT `FK_9c1af359bf6af1aebeea653b0ed`
        FOREIGN KEY (parent_id)
            REFERENCES ingredient (id)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB
    AUTO_INCREMENT = 3
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table user
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS user
(
    id          INT(11)      NOT NULL AUTO_INCREMENT,
    birthday    DATETIME     NOT NULL,
    create_time DATETIME(6)  NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    gender      VARCHAR(8)   NOT NULL,
    nickname    VARCHAR(16)  NOT NULL,
    password    VARCHAR(255) NOT NULL,
    username    VARCHAR(255) NOT NULL UNIQUE,
    user_tag    INT(11)      NOT NULL,
    photo_link  VARCHAR(255) NULL     DEFAULT NULL,
    PRIMARY KEY (id)
)
    ENGINE = InnoDB
    AUTO_INCREMENT = 30
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table post
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS post
(
    id          INT(11)      NOT NULL AUTO_INCREMENT,
    create_time DATETIME(6)  NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    title       VARCHAR(255) NOT NULL,
    user_id     INT(11)      NOT NULL,
    PRIMARY KEY (id),
    INDEX `FK_5c1cf55c308037b5aca1038a131` (user_id ASC),
    CONSTRAINT `FK_5c1cf55c308037b5aca1038a131`
        FOREIGN KEY (user_id)
            REFERENCES user (id)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table post_content
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS post_content
(
    id         INT(11)      NOT NULL AUTO_INCREMENT,
    content    VARCHAR(255) NOT NULL,
    photo_link VARCHAR(255) NOT NULL,
    post_id    INT(11)      NOT NULL,
    PRIMARY KEY (id),
    INDEX `FK_d5e71a3e4c6277474e6af05b8b2` (post_id ASC),
    CONSTRAINT `FK_d5e71a3e4c6277474e6af05b8b2`
        FOREIGN KEY (post_id)
            REFERENCES post (id)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table post_cookery
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS post_cookery
(
    post_id    INT(11) NOT NULL,
    cookery_id INT(11) NOT NULL,
    PRIMARY KEY (post_id, cookery_id),
    INDEX `IDX_b5109155d262c0accee8a0a5da` (post_id ASC),
    INDEX `IDX_9e089190f0c7f9e7ee586d1c07` (cookery_id ASC),
    CONSTRAINT `FK_9e089190f0c7f9e7ee586d1c076`
        FOREIGN KEY (cookery_id)
            REFERENCES cookery (id)
            ON DELETE CASCADE
            ON UPDATE NO ACTION,
    CONSTRAINT `FK_b5109155d262c0accee8a0a5da1`
        FOREIGN KEY (post_id)
            REFERENCES post (id)
            ON DELETE CASCADE
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table post_food_type
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS post_food_type
(
    post_id      INT(11) NOT NULL,
    food_type_id INT(11) NOT NULL,
    PRIMARY KEY (post_id, food_type_id),
    INDEX `IDX_32954178aad2d257898d82e598` (post_id ASC),
    INDEX `IDX_74da3c6ddf988eba0db8ae2ca2` (food_type_id ASC),
    CONSTRAINT `FK_32954178aad2d257898d82e598d`
        FOREIGN KEY (post_id)
            REFERENCES post (id)
            ON DELETE CASCADE
            ON UPDATE NO ACTION,
    CONSTRAINT `FK_74da3c6ddf988eba0db8ae2ca2b`
        FOREIGN KEY (food_type_id)
            REFERENCES food_type (id)
            ON DELETE CASCADE
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table post_ingredient
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS post_ingredient
(
    post_id       INT(11)      NOT NULL,
    ingredient_id INT(11)      NOT NULL,
    amount        INT(11)      NOT NULL,
    unit          VARCHAR(255) NOT NULL,
    PRIMARY KEY (post_id, ingredient_id),
    INDEX `FK_70e062685b27fccd2dac931581a` (ingredient_id ASC),
    CONSTRAINT `FK_70e062685b27fccd2dac931581a`
        FOREIGN KEY (ingredient_id)
            REFERENCES ingredient (id)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `FK_b9f502208ebb1947c1da2c4975e`
        FOREIGN KEY (post_id)
            REFERENCES post (id)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table vendor
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS vendor
(
    id   INT(11)      NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table seller
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS seller
(
    id   INT(11)      NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table product
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS product
(
    id                        VARCHAR(255) NOT NULL,
    info                      VARCHAR(255) NOT NULL,
    kan_product_category_code VARCHAR(255) NOT NULL,
    name                      VARCHAR(255) NOT NULL,
    photo_link                VARCHAR(255) NULL DEFAULT NULL,
    unit                      VARCHAR(255) NOT NULL,
    wight                     VARCHAR(255) NOT NULL,
    seller_id                 INT(11)      NOT NULL,
    vendor_id                 INT(11)      NOT NULL,
    PRIMARY KEY (id),
    INDEX `FK_d5cac481d22dacaf4d53f900a3f` (seller_id ASC),
    INDEX `FK_921582066aa70b502e78ea92012` (vendor_id ASC),
    CONSTRAINT `FK_921582066aa70b502e78ea92012`
        FOREIGN KEY (vendor_id)
            REFERENCES vendor (id)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `FK_d5cac481d22dacaf4d53f900a3f`
        FOREIGN KEY (seller_id)
            REFERENCES seller (id)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table post_product
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS post_product
(
    post_id    INT(11)      NOT NULL,
    product_id VARCHAR(255) NOT NULL,
    PRIMARY KEY (post_id, product_id),
    INDEX `IDX_72e185a9d26cfa23c3648ba854` (post_id ASC),
    INDEX `IDX_1faaa6e24261b54d14e1067844` (product_id ASC),
    CONSTRAINT `FK_1faaa6e24261b54d14e10678449`
        FOREIGN KEY (product_id)
            REFERENCES product (id)
            ON DELETE CASCADE
            ON UPDATE NO ACTION,
    CONSTRAINT `FK_72e185a9d26cfa23c3648ba8545`
        FOREIGN KEY (post_id)
            REFERENCES post (id)
            ON DELETE CASCADE
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB
    DEFAULT CHARACTER SET = utf8mb4;


-- -----------------------------------------------------
-- Table device
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS device
(
    id      INT(11)      NOT NULL AUTO_INCREMENT,
    token   VARCHAR(255) NOT NULL UNIQUE,
    name    VARCHAR(64)  NOT NULL,
    user_id INT(11)      NOT NULL,
    PRIMARY KEY (id),
    UNIQUE INDEX `REL_94f168faad896c0786646fa3d4` (user_id ASC),
    CONSTRAINT `FK_94f168faad896c0786646fa3d4a`
        FOREIGN KEY (user_id)
            REFERENCES user (id)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB
    AUTO_INCREMENT = 30
    DEFAULT CHARACTER SET = utf8mb4;


SET SQL_MODE = @OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS = @OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS = @OLD_UNIQUE_CHECKS;
