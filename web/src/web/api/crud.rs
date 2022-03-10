use std::collections::HashMap;
use std::mem::swap;

use poem::error::{BadRequest, InternalServerError};
use poem::web::Data;
use poem::Result;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use serde_json::{Map, Value};
use sqlx::database::HasArguments;
use sqlx::mysql::MySqlRow;
use sqlx::query::Query;
use sqlx::types::time::{Date, PrimitiveDateTime, Time};
use sqlx::types::BigDecimal;
use sqlx::{Column, MySql, Row};
use tokio_stream::StreamExt;

use crate::error::TransError::{
    CrudInfoNotFound, RequestMustBeJsonObject, RequestMustContain, RequestValueMustBeString,
};
use crate::web::api::Api;
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

#[OpenApi]
impl CRUDApi {
    #[oai(path = "/create/:table_name", method = "post", tag = "Api::CRUDApi")]
    async fn create(
        &self,
        pool: Data<&DbPool>,
        table_name: Path<String>,
        req: Json<Value>,
    ) -> Result<Json<u64>> {
        let req = get_object(req.0)?;
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

        values_sql.pop();
        values_sql += ")";
        sql.pop();
        sql += ") VALUES ";
        sql += &values_sql;
        let mut query = sqlx::query(&*sql);
        query = query_bind_value(query, req, placeholders)?;
        let res = query.execute(pool.0).await.map_err(InternalServerError)?;

        Ok(Json(res.rows_affected()))
    }

    #[oai(path = "/read/:table_name", method = "post", tag = "Api::CRUDApi")]
    async fn read(
        &self,
        pool: Data<&DbPool>,
        table_name: Path<String>,
        req: Json<Value>,
    ) -> Result<Json<Value>> {
        let req = get_object(req.0)?;
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
        query = query_bind_value(query, req, placeholders)?;
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

    #[oai(path = "/update/:table_name", method = "put", tag = "Api::CRUDApi")]
    async fn update(
        &self,
        pool: Data<&DbPool>,
        table_name: Path<String>,
        mut req: Json<Vec<Value>>,
    ) -> Result<Json<u64>> {
        let mut where_req = req.pop();
        let mut update_req = req.pop();
        if where_req.is_none() {
            return Err(BadRequest(RequestMustContain(
                "columns to update".to_string(),
            )));
        }

        let (update_req, where_req) = if update_req.is_none() {
            swap(&mut update_req, &mut where_req);
            (get_object(update_req.unwrap())?, None)
        } else {
            (
                get_object(update_req.unwrap())?,
                Some(get_object(where_req.unwrap())?),
            )
        };

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
                if update_req.contains_key(&column_name) {
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

                if where_req.is_none() {
                    break;
                }

                where_req
                    .as_ref()
                    .map(|req| {
                        if req.contains_key(&column_name) {
                            sql += &column_name;
                            sql += " = ? AND ";
                            where_placeholders.push((column_name, column_type));
                        } else if column_must {
                            return Err(BadRequest(RequestMustContain(column_name.to_string())));
                        }
                        Ok(())
                    })
                    .unwrap()?;
            }
        }

        if last_crud_type == CRUD_U {
            sql.pop();
        }
        sql = sql
            .trim_end_matches("WHERE ")
            .trim_end_matches("AND ")
            .to_string();

        let mut query = sqlx::query(&*sql);
        query = query_bind_value(query, update_req, update_placeholders)?;
        if let Some(where_req) = where_req {
            query = query_bind_value(query, where_req, where_placeholders)?;
        }

        let res = query.execute(pool.0).await.map_err(InternalServerError)?;

        Ok(Json(res.rows_affected()))
    }

    #[oai(path = "/delete/:table_name", method = "delete", tag = "Api::CRUDApi")]
    async fn delete(
        &self,
        pool: Data<&DbPool>,
        table_name: Path<String>,
        req: Json<Value>,
    ) -> Result<Json<u64>> {
        let req = get_object(req.0)?;
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
        query = query_bind_value(query, req, placeholders)?;
        let res = query.execute(pool.0).await.map_err(InternalServerError)?;

        Ok(Json(res.rows_affected()))
    }
}

fn get_object(req: Value) -> Result<Map<String, Value>> {
    match req {
        Value::Object(r) => Ok(r),
        _ => Err(BadRequest(RequestMustBeJsonObject)),
    }
}

fn query_bind_value<'a>(
    mut query: Query<'a, MySql, <MySql as HasArguments>::Arguments>,
    mut req: Map<String, Value>,
    columns: Vec<(String, String)>,
) -> Result<Query<'a, MySql, <MySql as HasArguments<'a>>::Arguments>> {
    for (column_name, column_type) in columns {
        match req.remove(&column_name).unwrap() {
            Value::String(s) => query = bind_value(query, s, column_type),
            _ => return Err(BadRequest(RequestValueMustBeString(column_name))),
        }
    }

    Ok(query)
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
        "DATE" => match Date::parse(&*value, "%F") {
            Ok(v) => query.bind(v),
            Err(e) => {
                tracing::error!("{} parse Date failed: {}", value, e);
                query
            }
        },
        "DATETIME" => match PrimitiveDateTime::parse(&*value, "%F %T") {
            Ok(v) => query.bind(v),
            Err(_) => {
                tracing::error!("{} parse DateTime failed", value);
                query
            }
        },
        "DECIMAL" => query.bind(value.parse::<BigDecimal>().unwrap_or_default()),
        "DOUBLE" => query.bind(value.parse::<f64>().unwrap_or_default()),
        "FLOAT" => query.bind(value.parse::<f32>().unwrap_or_default()),
        "INT" => query.bind(value.parse::<i32>().unwrap_or_default()),
        "INT_UNSIGNED" => query.bind(value.parse::<u32>().unwrap_or_default()),
        "SMALLINT" => query.bind(value.parse::<i16>().unwrap_or_default()),
        "SMALLINT_UNSIGNED" => query.bind(value.parse::<u16>().unwrap_or_default()),
        "TEXT" => query.bind(value),
        "TIME" => match Time::parse(&*value, "%T") {
            Ok(v) => query.bind(v),
            Err(_) => {
                tracing::error!("{} parse Time failed", value);
                query
            }
        },
        "TINYINT" => query.bind(value.parse::<i8>().unwrap_or_default()),
        "TINYINT_UNSIGNED" => query.bind(value.parse::<u8>().unwrap_or_default()),
        _ => query,
    }
}
