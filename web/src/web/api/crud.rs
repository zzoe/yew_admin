use std::collections::HashMap;

use poem::error::{BadRequest, InternalServerError};
use poem::web::Data;
use poem::Result;
use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi};
use serde_json::Value;
use sqlx::database::HasArguments;
use sqlx::mysql::MySqlRow;
use sqlx::query::Query;
use sqlx::types::time::{Date, PrimitiveDateTime, Time};
use sqlx::types::BigDecimal;
use sqlx::{Column, MySql, Row};
use time::format_description;
use tokio_stream::StreamExt;

use crate::error::TransError::{CrudInfoNotFound, RequestMustContain};
use crate::web::routes::DbPool;

pub(crate) struct CRUDApi;

const SQL_CRUD_TYPE_EQ: &str =
    "SELECT i.`column_name`,i.`column_must`,t.`column_type` FROM CRUD_INFO i left join table_columns t on i.`table_name`=t.`table_name` and i.`column_name` = t.`column_name` WHERE i.`table_name` = ? and i.`crud_type` = ? order by i.`column_order`";

const SQL_CRUD_TYPE_IN: &str =
    "SELECT i.`crud_type`,i.`column_name`,i.`column_must`,t.`column_type` FROM CRUD_INFO i left join table_columns t on i.`table_name`=t.`table_name` and i.`column_name` = t.`column_name` WHERE i.`table_name` = ? and i.`crud_type` in (?,?) order by i.`crud_type`,i.`column_order`";

// 新增
const CRUD_C: &str = "c";
// 查询结果
const CRUD_R: &str = "r";
// 查询条件
const CRUD_S: &str = "s";
// 更新字段
const CRUD_U: &str = "u";
// 更新条件
const CRUD_V: &str = "v";
// 删除条件
const CRUD_D: &str = "d";

#[derive(Object)]
struct CRUDInfo {
    table_name: String,
    columns: HashMap<String, String>,
    conditions: HashMap<String, String>,
}

#[OpenApi]
impl CRUDApi {
    #[oai(path = "/create", method = "post")]
    async fn create(&self, pool: Data<&DbPool>, req: Json<CRUDInfo>) -> Result<Json<u64>> {
        let table_name = req.0.table_name;
        let req = req.0.columns;
        if req.is_empty() {
            return Err(BadRequest(RequestMustContain(
                "columns to create".to_string(),
            )));
        }

        let mut sql = format!("INSERT INTO {}(", &*table_name);
        let mut values_sql = String::from("(");
        let mut placeholders = Vec::new();
        let mut insert_stream = sqlx::query_as::<_, (String, bool, String)>(SQL_CRUD_TYPE_EQ)
            .bind(&*table_name)
            .bind(CRUD_C)
            .fetch(pool.0);

        while let Some(a) = insert_stream.next().await {
            let (column_name, column_must, column_type) = a.map_err(InternalServerError)?;

            if req.contains_key(&*column_name) {
                sql += &*column_name;
                sql += ",";
                values_sql += "?,";
                placeholders.push((column_name, column_type));
            } else if column_must {
                return Err(BadRequest(RequestMustContain(column_name.to_string())));
            }
        }

        if placeholders.is_empty() {
            return Err(BadRequest(RequestMustContain(
                "columns to create".to_string(),
            )));
        }

        values_sql.pop();
        values_sql += ")";
        sql.pop();
        sql += ") VALUES ";
        sql += &values_sql;
        let mut query = sqlx::query(&*sql);
        query = query_bind_value(query, req, placeholders);
        let res = query.execute(pool.0).await.map_err(InternalServerError)?;

        Ok(Json(res.rows_affected()))
    }

    #[oai(path = "/read", method = "post")]
    async fn read(&self, pool: Data<&DbPool>, req: Json<CRUDInfo>) -> Result<Json<Value>> {
        let table_name = req.0.table_name;
        let req = req.0.conditions;

        let mut sql = String::from("SELECT ");
        let mut select_columns = HashMap::new();
        let mut placeholders = Vec::new();
        let mut last_crud_type = "";
        let mut select_stream =
            sqlx::query_as::<_, (String, String, bool, String)>(SQL_CRUD_TYPE_IN)
                .bind(&*table_name)
                .bind(CRUD_R)
                .bind(CRUD_S)
                .fetch(pool.0);

        while let Some(res) = select_stream.next().await {
            let (crud_type, column_name, column_must, column_type) =
                res.map_err(InternalServerError)?;

            if last_crud_type.is_empty() && crud_type != CRUD_R {
                return Err(BadRequest(CrudInfoNotFound(
                    table_name.to_string(),
                    String::from(CRUD_R),
                )));
            }

            if crud_type == CRUD_R {
                select_columns.insert(column_name.clone(), column_type);
                sql += &column_name;
                sql += ",";
                last_crud_type = CRUD_R;
            } else if crud_type == CRUD_S {
                if last_crud_type == CRUD_R {
                    sql.pop();
                    sql += " FROM ";
                    sql += &*table_name;
                    sql += " WHERE ";
                    last_crud_type = CRUD_S;
                }

                if req.contains_key(&column_name) {
                    sql += &column_name;
                    sql += " = ? AND ";
                    placeholders.push((column_name, column_type));
                } else if column_must {
                    return Err(BadRequest(RequestMustContain(table_name.to_string())));
                }
            }
        }

        if select_columns.is_empty() {
            return Err(BadRequest(RequestMustContain(
                "columns to read".to_string(),
            )));
        }

        if last_crud_type == CRUD_R {
            sql.pop();
            sql += " FROM ";
            sql += &*table_name;
        }

        sql = sql
            .trim_end_matches("WHERE ")
            .trim_end_matches("AND ")
            .to_string();

        let mut rows = Vec::new();
        let mut query = sqlx::query(&*sql);
        query = query_bind_value(query, req, placeholders);
        let mut stream = query.fetch(pool.0);
        while let Some(res) = stream.next().await {
            let row = res.map_err(InternalServerError)?;
            let mut r = serde_json::Map::new();

            row.columns().iter().enumerate().for_each(|(i, col)| {
                let column_type = select_columns.get(col.name()).unwrap();
                r.insert(col.name().to_string(), get_value(&row, i, column_type));
            });

            rows.push(Value::Object(r));
        }

        Ok(Json(Value::Array(rows)))
    }

    #[oai(path = "/update", method = "put")]
    async fn update(&self, pool: Data<&DbPool>, req: Json<CRUDInfo>) -> Result<Json<u64>> {
        let table_name = req.0.table_name;
        let update_columns = req.0.columns;
        let conditions = req.0.conditions;
        if update_columns.is_empty() {
            return Err(BadRequest(RequestMustContain(
                "columns to update".to_string(),
            )));
        }

        let mut sql = format!("UPDATE {} SET ", &*table_name);
        let mut update_placeholders = Vec::new();
        let mut where_placeholders = Vec::new();
        let mut last_crud_type = "";
        let mut update_stream =
            sqlx::query_as::<_, (String, String, bool, String)>(SQL_CRUD_TYPE_IN)
                .bind(&*table_name)
                .bind(CRUD_U)
                .bind(CRUD_V)
                .fetch(pool.0);

        while let Some(res) = update_stream.next().await {
            let (crud_type, column_name, column_must, column_type) =
                res.map_err(InternalServerError)?;

            if last_crud_type.is_empty() && crud_type != CRUD_U {
                return Err(BadRequest(CrudInfoNotFound(
                    table_name.to_string(),
                    String::from(CRUD_U),
                )));
            }

            if crud_type == CRUD_U {
                if update_columns.contains_key(&column_name) {
                    sql += &column_name;
                    sql += " = ?,";
                    last_crud_type = CRUD_U;
                    update_placeholders.push((column_name, column_type));
                } else if column_must {
                    return Err(BadRequest(RequestMustContain(column_name.to_string())));
                }
            } else if crud_type == CRUD_V {
                if last_crud_type == CRUD_U {
                    sql.pop();
                    sql += " WHERE ";
                    last_crud_type = CRUD_V;
                } else if last_crud_type.is_empty() {
                    return Err(BadRequest(CrudInfoNotFound(
                        table_name.to_string(),
                        String::from(CRUD_U),
                    )));
                }

                if conditions.is_empty() {
                    break;
                }

                if conditions.contains_key(&column_name) {
                    sql += &column_name;
                    sql += " = ? AND ";
                    where_placeholders.push((column_name, column_type));
                } else if column_must {
                    return Err(BadRequest(RequestMustContain(column_name.to_string())));
                }
            }
        }

        if update_placeholders.is_empty() {
            return Err(BadRequest(RequestMustContain(
                "columns to update".to_string(),
            )));
        }

        if last_crud_type == CRUD_U {
            sql.pop();
        }
        sql = sql
            .trim_end_matches("WHERE ")
            .trim_end_matches("AND ")
            .to_string();

        let mut query = sqlx::query(&*sql);
        query = query_bind_value(query, update_columns, update_placeholders);
        query = query_bind_value(query, conditions, where_placeholders);

        let res = query.execute(pool.0).await.map_err(InternalServerError)?;

        Ok(Json(res.rows_affected()))
    }

    #[oai(path = "/delete", method = "delete")]
    async fn delete(&self, pool: Data<&DbPool>, req: Json<CRUDInfo>) -> Result<Json<u64>> {
        let table_name = req.0.table_name;
        let req = req.0.conditions;
        let mut sql = format!("DELETE FROM {} WHERE ", &*table_name);
        let mut placeholders = Vec::new();
        let mut delete_stream = sqlx::query_as::<_, (String, bool, String)>(SQL_CRUD_TYPE_EQ)
            .bind(&*table_name)
            .bind(CRUD_D)
            .fetch(pool.0);

        while let Some(a) = delete_stream.next().await {
            let (column_name, column_must, column_type) = a.map_err(InternalServerError)?;

            if req.contains_key(&*column_name) {
                sql += &*column_name;
                sql += " = ? AND ";
                placeholders.push((column_name, column_type));
            } else if column_must {
                return Err(BadRequest(RequestMustContain(column_name.to_string())));
            }
        }

        sql = sql
            .trim_end_matches("WHERE ")
            .trim_end_matches("AND ")
            .to_string();

        let mut query = sqlx::query(&*sql);
        query = query_bind_value(query, req, placeholders);
        let res = query.execute(pool.0).await.map_err(InternalServerError)?;

        Ok(Json(res.rows_affected()))
    }
}

fn query_bind_value<'a>(
    mut query: Query<'a, MySql, <MySql as HasArguments>::Arguments>,
    mut req: HashMap<String, String>,
    columns: Vec<(String, String)>,
) -> Query<'a, MySql, <MySql as HasArguments<'a>>::Arguments> {
    for (column_name, column_type) in columns {
        let column_value = req.remove(&*column_name);
        if let Some(column_value) = column_value {
            query = bind_value(query, column_value, column_type);
        }
    }

    query
}

fn get_value(row: &MySqlRow, i: usize, value_type: &str) -> Value {
    let value = match value_type {
        "BIGINT" => row
            .try_get::<i64, usize>(i)
            .map(|v| v.to_string())
            .unwrap_or_default(),
        "BIGINT_UNSIGNED" => row
            .try_get::<u64, usize>(i)
            .map(|v| v.to_string())
            .unwrap_or_default(),
        "BLOB" => String::from_utf8(row.try_get::<Vec<u8>, usize>(i).unwrap_or_default())
            .unwrap_or_default(),
        "BOOLEAN" => row
            .try_get::<bool, usize>(i)
            .map(|v| v.to_string())
            .unwrap_or_default(),
        "DATE" => row
            .try_get::<Date, usize>(i)
            .map(|v| v.to_string())
            .unwrap_or_default(),
        "DATETIME" => row
            .try_get::<PrimitiveDateTime, usize>(i)
            .map(|v| v.to_string())
            .unwrap_or_default(),
        "DECIMAL" => row
            .try_get::<BigDecimal, usize>(i)
            .map(|v| v.to_string())
            .unwrap_or_default(),
        "DOUBLE" => row
            .try_get::<f64, usize>(i)
            .map(|v| v.to_string())
            .unwrap_or_default(),
        "FLOAT" => row
            .try_get::<f32, usize>(i)
            .map(|v| v.to_string())
            .unwrap_or_default(),
        "INT" => row
            .try_get::<i32, usize>(i)
            .map(|v| v.to_string())
            .unwrap_or_default(),
        "INT_UNSIGNED" => row
            .try_get::<u32, usize>(i)
            .map(|v| v.to_string())
            .unwrap_or_default(),
        "SMALLINT" => row
            .try_get::<i16, usize>(i)
            .map(|v| v.to_string())
            .unwrap_or_default(),
        "SMALLINT_UNSIGNED" => row
            .try_get::<u16, usize>(i)
            .map(|v| v.to_string())
            .unwrap_or_default(),
        "TEXT" => row.try_get::<String, usize>(i).unwrap_or_default(),
        "TIME" => row
            .try_get::<Time, usize>(i)
            .map(|v| v.to_string())
            .unwrap_or_default(),
        "TINYINT" => row
            .try_get::<i8, usize>(i)
            .map(|v| v.to_string())
            .unwrap_or_default(),
        "TINYINT_UNSIGNED" => row
            .try_get::<u8, usize>(i)
            .map(|v| v.to_string())
            .unwrap_or_default(),
        _ => String::new(),
    };
    Value::String(value)
}

fn bind_value<'a>(
    query: Query<'a, MySql, <MySql as HasArguments>::Arguments>,
    value: String,
    value_type: String,
) -> Query<'a, MySql, <MySql as HasArguments<'a>>::Arguments> {
    match value_type.as_str() {
        "BIGINT" => query.bind(value.parse::<i64>().unwrap_or_default()),
        "BIGINT_UNSIGNED" => query.bind(value.parse::<u64>().unwrap_or_default()),
        "BLOB" => query.bind(value),
        "BOOLEAN" => query.bind(value.parse::<bool>().unwrap_or_default()),
        "DATE" => {
            let format = format_description::parse("%F").unwrap();
            match Date::parse(&*value, &format) {
                Ok(v) => query.bind(v),
                Err(e) => {
                    tracing::error!("{} parse Date failed: {}", value, e);
                    query
                }
            }
        }
        "DATETIME" => {
            let format = format_description::parse("%F %T").unwrap();
            match PrimitiveDateTime::parse(&*value, &format) {
                Ok(v) => query.bind(v),
                Err(_) => {
                    tracing::error!("{} parse DateTime failed", value);
                    query
                }
            }
        }
        "DECIMAL" => query.bind(value.parse::<BigDecimal>().unwrap_or_default()),
        "DOUBLE" => query.bind(value.parse::<f64>().unwrap_or_default()),
        "FLOAT" => query.bind(value.parse::<f32>().unwrap_or_default()),
        "INT" => query.bind(value.parse::<i32>().unwrap_or_default()),
        "INT_UNSIGNED" => query.bind(value.parse::<u32>().unwrap_or_default()),
        "SMALLINT" => query.bind(value.parse::<i16>().unwrap_or_default()),
        "SMALLINT_UNSIGNED" => query.bind(value.parse::<u16>().unwrap_or_default()),
        "TEXT" => query.bind(value),
        "TIME" => {
            let format = format_description::parse("%T").unwrap();
            match Time::parse(&*value, &format) {
                Ok(v) => query.bind(v),
                Err(_) => {
                    tracing::error!("{} parse Time failed", value);
                    query
                }
            }
        }
        "TINYINT" => query.bind(value.parse::<i8>().unwrap_or_default()),
        "TINYINT_UNSIGNED" => query.bind(value.parse::<u8>().unwrap_or_default()),
        _ => query,
    }
}
