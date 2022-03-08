DROP TABLE IF EXISTS `menu_info`;
CREATE TABLE `menu_info`
(
    `MENU_ID`    INT          NOT NULL AUTO_INCREMENT COMMENT '菜单编号',
    `PARENT_ID`  INT          NOT NULL COMMENT '上级菜单编号',
    `MENU_TYPE`  INT          NOT NULL COMMENT '菜单类型 1-Label 2-Fold 3-Item',
    `MENU_NAME`  VARCHAR(255) NOT NULL COMMENT '菜单名称',
    `PAGE_ID`    INT          NOT NULL DEFAULT 0 COMMENT '页面编号',
    `TABLE_NAME` VARCHAR(255) COMMENT '表名',
    `MENU_ORDER` INT          NOT NULL DEFAULT 0 COMMENT '菜单顺序',
    PRIMARY KEY (`MENU_ID`),
    INDEX `IDX_MENU_ORDER` (`PARENT_ID`, `MENU_ORDER`),
    INDEX `IDX_MENU_PAGE` (`PAGE_ID`),
    FULLTEXT INDEX `IDX_MENU_NAME` (`MENU_NAME`)
) COMMENT = '菜单信息表';

DROP TABLE IF EXISTS `crud_info`;
CREATE TABLE `crud_info`
(
    `TABLE_NAME`   VARCHAR(255) NOT NULL COMMENT '表名',
    `CRUD_TYPE`    CHAR         NOT NULL COMMENT 'CRUD类型 c-create r-read s-readCondition u-update v-updateCondition d-deleteCondition',
    `COLUMN_NAME`  VARCHAR(255) NOT NULL COMMENT '列名',
    `COLUMN_MUST`  BOOLEAN      NOT NULL COMMENT '是否必填',
    `COLUMN_ORDER` INT          NOT NULL COMMENT '字段顺序',
    PRIMARY KEY (`TABLE_NAME`, `CRUD_TYPE`, `COLUMN_NAME`),
    INDEX `IDX_CRUD_ORDER` (`TABLE_NAME`, `CRUD_TYPE`, `COLUMN_ORDER`)
) COMMENT = 'CRUD信息表';

/**
 * BIGINT                   i64
 * BIGINT_UNSIGNED          u64
 * BLOB                     &[u8], Vec<u8>
 * BOOLEAN                  bool
 * DATE                     time::Date
 * DATETIME                 time::PrimitiveDateTime
 * DECIMAL                  bigdecimal::BigDecimal
 * DOUBLE                   f64
 * FLOAT                    f32
 * INT                      i32
 * INT_UNSIGNED             u32
 * SMALLINT                 i16
 * SMALLINT_UNSIGNED        u16
 * TEXT                     String
 * TIME                     time::Time
 * TINYINT                  i8
 * TINYINT_UNSIGNED         u8
  */
DROP TABLE IF EXISTS `table_columns`;
CREATE TABLE `table_columns`
(
    `TABLE_NAME`   VARCHAR(255) NOT NULL COMMENT '表名',
    `COLUMN_NAME`  VARCHAR(255) NOT NULL COMMENT '列名',
    `COLUMN_STYLE` INT          NOT NULL COMMENT '字段样式 1-input 2-select 3-checkbox 4-radio 5-date 6-time 7-datetime 8-textarea',
    `COLUMN_TYPE`  VARCHAR(24)  NOT NULL COMMENT '字段类型 BIGINT, BIGINT_UNSIGNED, BLOB, BOOLEAN, DATE, DATETIME, DECIMAL, DOUBLE, FLOAT, INT, INT_UNSIGNED, SMALLINT, SMALLINT_UNSIGNED, TEXT, TIME, TINYINT, TINYINT_UNSIGNED',
    `COLUMN_DICT`  INT COMMENT '字典ID',
    `COLUMN_DESC`  VARCHAR(255) COMMENT '列描述',
    PRIMARY KEY (TABLE_NAME, `COLUMN_NAME`)
) COMMENT = '更新字段';

DROP TABLE IF EXISTS `dict_info`;
CREATE TABLE `dict_info`
(
    `DICT_ID`   INT          NOT NULL COMMENT '字典编号',
    `DICT_NAME` VARCHAR(255) NOT NULL COMMENT '字典名称',
    `KEY_ORDER` INT          NOT NULL COMMENT '字典KEY顺序',
    `KEY`       VARCHAR(255) NOT NULL COMMENT '字典KEY',
    `VALUE`     VARCHAR(255) NOT NULL COMMENT '字典值',
    `DESC`      VARCHAR(255) COMMENT '字典描述',
    PRIMARY KEY (`DICT_ID`, `KEY`),
    INDEX `IDX_DICT_ORDER` (`DICT_ID`, `KEY_ORDER`)
) COMMENT = '字典信息表';
