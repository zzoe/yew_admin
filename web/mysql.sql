DROP TABLE IF EXISTS `menu_info`;
CREATE TABLE `menu_info`
(
    `MENU_ID`       INT          NOT NULL AUTO_INCREMENT COMMENT '菜单编号',
    `PARENT_ID`     INT          NOT NULL COMMENT '上级菜单编号',
    `MENU_TYPE`     INT          NOT NULL COMMENT '菜单类型 1-Label 2-Fold 3-Item',
    `MENU_NAME`     VARCHAR(255) NOT NULL COMMENT '菜单名称',
    `FUNCTION_TYPE` INT          NOT NULL COMMENT '功能类型 1-crud',
    `FUNCTION_ID`   INT          NOT NULL DEFAULT 0 COMMENT '菜单功能ID',
    `MENU_ORDER`    INT          NOT NULL DEFAULT 0 COMMENT '菜单顺序',
    PRIMARY KEY (`MENU_ID`)
) COMMENT = '菜单信息表';

DROP TABLE IF EXISTS `crud_list`;
CREATE TABLE `crud_list`
(
    `FUNCTION_ID` INT          NOT NULL AUTO_INCREMENT COMMENT '功能编号',
    `TABLE_NAME`  VARCHAR(255) NOT NULL COMMENT '表名',
    `SELECT_ID`   INT          NOT NULL COMMENT '查询结果',
    `WHERE_ID`    INT          NOT NULL COMMENT '查询条件',
    `EDIT_ID`     INT          NOT NULL COMMENT '更新字段',
    PRIMARY KEY (`FUNCTION_ID`)
) COMMENT = '增删改查功能清单';

DROP TABLE IF EXISTS `crud_select`;
CREATE TABLE `crud_select`
(
    `TENANT_ID`   INT          NOT NULL AUTO_INCREMENT COMMENT '租户号',
    `FUNCTION_ID` INT          NOT NULL COMMENT '功能编号',
    `SELECT_ID`   INT          NOT NULL COMMENT '查询结果',
    `COLUMN_NAME` VARCHAR(255) NOT NULL COMMENT '列名',
    `COL_DESC`    VARCHAR(255) COMMENT '列描述',
    `COL_ORDER`   INT          NOT NULL COMMENT '顺序',
    PRIMARY KEY (`TENANT_ID`)
) COMMENT = '查询结果';

DROP TABLE IF EXISTS `crud_where`;
CREATE TABLE `crud_where`
(
    `TENANT_ID`    INT          NOT NULL AUTO_INCREMENT COMMENT '租户号',
    `FUNCTION_ID`  INT          NOT NULL COMMENT '功能编号',
    `WHERE_ID`     INT          NOT NULL COMMENT '查询条件',
    `COLUMN_NAME`  VARCHAR(255) NOT NULL COMMENT '列名',
    `COLUMN_MUST`  BOOLEAN      NOT NULL COMMENT '是否必输',
    `COLUMN_STYLE` INT          NOT NULL COMMENT '字段样式 1-input 2-textarea 3-select 4-checkbox 5-radio 6-date 7-time 8-datetime 9-file',
    `COLUMN_TYPE`  INT          NOT NULL COMMENT '字段类型 1-字符串 2-数字 3-日期',
    `COLUMN_DICT`  INT COMMENT '字典ID',
    `COLUMN_DESC`  VARCHAR(255) COMMENT '列描述',
    `COLUMN_ORDER` INT          NOT NULL COMMENT '顺序',
    PRIMARY KEY (`TENANT_ID`)
) COMMENT = '查询条件';

DROP TABLE IF EXISTS `crud_edit`;
CREATE TABLE `crud_edit`
(
    `TENANT_ID`    INT          NOT NULL AUTO_INCREMENT COMMENT '租户号',
    `FUNCTION_ID`  INT          NOT NULL COMMENT '功能编号',
    `EDIT_ID`      INT          NOT NULL COMMENT '查询条件',
    `COLUMN_NAME`  VARCHAR(255) NOT NULL COMMENT '列名',
    `COLUMN_MUST`  BOOLEAN      NOT NULL COMMENT '是否必输',
    `COLUMN_STYLE` INT          NOT NULL COMMENT '字段样式 1-input 2-textarea 3-select 4-checkbox 5-radio 6-date 7-time 8-datetime 9-file',
    `COLUMN_TYPE`  INT          NOT NULL COMMENT '字段类型 1-字符串 2-日期 3-数字',
    `COLUMN_DICT`  INT COMMENT '字典ID',
    `COLUMN_DESC`  VARCHAR(255) COMMENT '列描述',
    `COLUMN_ORDER` INT          NOT NULL COMMENT '顺序',
    PRIMARY KEY (`TENANT_ID`)
) COMMENT = '更新字段';