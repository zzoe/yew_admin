use poem::error::{BadRequest, InternalServerError};
use poem::web::Data;
use poem::Result;
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use serde_json::Value;
use tokio_stream::StreamExt;

use crate::error::TransError::{
    RequestMustBeJsonObject, RequestMustContain, RequestMustContainTableName,
};
use crate::web::api::Api;
use crate::web::routes::DbPool;

pub(crate) struct CRUDApi;

const SQL_CRUD: &str =
    "SELECT i.`column_name`,i.`column_must`,t.`column_type` FROM CRUD_INFO i left join table_columns t on i.`table_name`=t.`table_name` WHERE i.`table_name` = ? and i.`crud_type` = ? order by i.`column_order`";

#[OpenApi]
impl CRUDApi {
    #[oai(path = "/crud/create", method = "post", tag = "Api::CRUDApi")]
    async fn create(&self, pool: Data<&DbPool>, req: Json<Value>) -> Result<Json<u64>> {
        Ok(Json(1))
    }

    #[oai(path = "/crud/read", method = "post", tag = "Api::CRUDApi")]
    async fn read(&self, pool: Data<&DbPool>, req: Json<Value>) -> Result<Json<Value>> {
        let req = match req.0 {
            Value::Object(r) => r,
            _ => return Err(BadRequest(RequestMustBeJsonObject)),
        };

        let table_name = match req.get("table_name") {
            Some(Value::String(s)) => s,
            _ => return Err(BadRequest(RequestMustContainTableName)),
        };

        let mut sql = String::from("SELECT ");

        let mut select_stream = sqlx::query_as::<_, (String, bool, String)>(SQL_CRUD)
            .bind(table_name)
            .bind("r")
            .fetch(pool.0);
        while let Some(res) = select_stream.next().await {
            let (column_name, _, column_type) = res.map_err(InternalServerError)?;
            sql += &column_name;
            sql += ",";
        }
        sql.pop();
        sql += " FROM ";
        sql += table_name;
        sql += " WHERE ";

        let mut where_columns = Vec::new();
        let mut stream = sqlx::query_as::<_, (String, bool, String)>(SQL_CRUD)
            .bind(table_name)
            .bind("s")
            .fetch(pool.0);
        while let Some(res) = stream.next().await {
            let (column_name, column_must, column_type) = res.map_err(InternalServerError)?;
            if req.contains_key(&column_name) {
                sql += &column_name;
                sql += " = ? AND ";
                where_columns.push((column_name, column_type));
            } else if column_must {
                return Err(BadRequest(RequestMustContain(table_name.to_string())));
            }
        }
        sql = sql.trim_end_matches("AND ").to_string();

        let mut rows = Vec::new();
        // let query = sqlx::query(&sql);
        // for (column_name, column_type) in where_columns {
        //     query.bind(req.get(&column).unwrap());
        // }
        // .bind(req.get(&where_columns[0]).unwrap())
        // .fetch_all(pool.0)
        // .for_each(|row| {
        //     rows.push(row);
        //     Ok(())
        // })
        // .await
        // .map_err(InternalServerError)?;

        Ok(Json(Value::Array(rows)))
    }

    #[oai(path = "/crud/update", method = "put", tag = "Api::CRUDApi")]
    async fn update(&self, pool: Data<&DbPool>, req: Json<Value>) -> Result<Json<u64>> {
        Ok(Json(1))
    }

    #[oai(path = "/crud/delete", method = "delete", tag = "Api::CRUDApi")]
    async fn delete(&self, pool: Data<&DbPool>, req: Json<Value>) -> Result<Json<u64>> {
        Ok(Json(1))
    }
}
