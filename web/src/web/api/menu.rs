use crate::web::DbPool;
use poem::error::InternalServerError;
use poem::web::Data;
use poem::Result;
use poem_openapi::param::Path;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{ApiResponse, Object, OpenApi};
use tokio_stream::StreamExt;

pub(crate) struct MenuApi;

const MENU_CREATE: &str = "insert into menu_info (parent_id, menu_type, menu_name, function_type, function_id) values (?, ?, ?, ?, ?)";
const READ_MENU :&str = "select menu_id, parent_id, menu_type, menu_name, function_type, function_id, menu_order from menu_info where menu_id = ?";
const READ_MENU_LIST :&str = "select menu_id, parent_id, menu_type, menu_name, function_type, function_id, menu_order from menu_info";

#[derive(Object, sqlx::FromRow)]
struct Menu {
    menu_id: i32,
    parent_id: i32,
    menu_type: i8,
    menu_name: String,
    function_type: i8,
    function_id: i32,
    menu_order: i32,
}

#[derive(Object)]
struct MenuReq {
    update_menu: Option<MenuOpt>,
    where_menu: Option<MenuOpt>,
}

#[derive(Object)]
struct MenuOpt {
    menu_id: Option<i32>,
    parent_id: Option<i32>,
    menu_type: Option<i8>,
    menu_name: Option<String>,
    function_type: Option<i8>,
    function_id: Option<i32>,
    menu_order: Option<i32>,
}

#[derive(ApiResponse)]
enum Response {
    #[oai(status = 200)]
    Menu(Json<Menu>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

#[OpenApi]
impl MenuApi {
    #[oai(path = "/menu", method = "post")]
    async fn create(&self, pool: Data<&DbPool>, menu: Json<Menu>) -> Result<Json<u64>> {
        let menu = menu.0;
        let id = sqlx::query(MENU_CREATE)
            .bind(menu.parent_id)
            .bind(menu.menu_type)
            .bind(menu.menu_name)
            .bind(menu.function_type)
            .bind(menu.function_id)
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?
            .last_insert_id();
        Ok(Json(id))
    }

    #[oai(path = "/menu/:id", method = "get")]
    async fn read(&self, pool: Data<&DbPool>, id: Path<u32>) -> Result<Response> {
        let menu: Option<Menu> = sqlx::query_as(READ_MENU)
            .bind(id.0)
            .fetch_optional(pool.0)
            .await
            .map_err(InternalServerError)?;

        match menu {
            Some(menu) => Ok(Response::Menu(Json(menu))),
            None => Ok(Response::NotFound(PlainText(format!(
                "Menu {} Not Found",
                id.0
            )))),
        }
    }

    #[oai(path = "/menu", method = "get")]
    async fn read_all(&self, pool: Data<&DbPool>) -> Result<Json<Vec<Menu>>> {
        let mut menus = Vec::new();
        let mut stream = sqlx::query_as::<_, Menu>(READ_MENU_LIST).fetch(pool.0);
        while let Some(res) = stream.next().await {
            menus.push(res.map_err(InternalServerError)?);
        }

        Ok(Json(menus))
    }

    #[oai(path = "/menu", method = "put")]
    async fn update(&self, pool: Data<&DbPool>, menu_req: Json<MenuReq>) -> Result<Json<u64>> {
        let mut sql = "update menu_info set ".to_string();
        if menu_req.update_menu.is_none() {
            return Ok(Json(0));
        }

        let update_menu = menu_req.update_menu.as_ref().unwrap();
        if update_menu.menu_name.is_some() {
            sql += "menu_name = ?,";
        }
        if update_menu.menu_order.is_some() {
            sql += "menu_order = ?,";
        }

        sql.pop();

        if menu_req.where_menu.is_some() {
            let mut and_str = " where ";
            let where_menu = menu_req.where_menu.as_ref().unwrap();

            if where_menu.menu_id.is_some() {
                sql += and_str;
                sql += "menu_id = ?";
                and_str = " and ";
            }

            if where_menu.menu_order.is_some() {
                sql += and_str;
                sql += "menu_order = ?";
            }
        }

        let mut query = sqlx::query(&sql);

        if update_menu.menu_name.is_some() {
            query = query.bind(update_menu.menu_name.clone().unwrap());
        }
        if update_menu.menu_order.is_some() {
            query = query.bind(update_menu.menu_order.unwrap());
        }

        if menu_req.where_menu.is_some() {
            let where_menu = menu_req.where_menu.as_ref().unwrap();

            if where_menu.menu_id.is_some() {
                query = query.bind(where_menu.menu_id.unwrap());
            }

            if where_menu.menu_order.is_some() {
                query = query.bind(where_menu.menu_order.unwrap());
            }
        }

        let count = query
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?
            .rows_affected();
        Ok(Json(count))
    }

    #[oai(path = "/menu", method = "delete")]
    async fn delete(&self, pool: Data<&DbPool>, delete_menu: Json<MenuOpt>) -> Result<Json<u64>> {
        let mut sql = "delete from menu_info ".to_string();
        let mut and_str = " where ";

        if delete_menu.menu_id.is_some() {
            sql += and_str;
            sql += "menu_id = ?";
            and_str = " and ";
        }

        if delete_menu.menu_order.is_some() {
            sql += and_str;
            sql += "menu_order = ?";
        }

        let mut query = sqlx::query(&sql);

        if delete_menu.menu_id.is_some() {
            query = query.bind(delete_menu.menu_id.unwrap());
        }

        if delete_menu.menu_order.is_some() {
            query = query.bind(delete_menu.menu_order.unwrap());
        }

        let count = query
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?
            .rows_affected();
        Ok(Json(count))
    }
}
