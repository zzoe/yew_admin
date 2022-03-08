use poem::Route;
use poem_openapi::registry::{MetaApi, Registry};
use poem_openapi::{OpenApi, Tags};

use web_derive::CombinedApi;

use crate::web::api::crud::CRUDApi;
use crate::web::api::menu::MenuApi;

pub(crate) mod crud;
pub(crate) mod menu;

#[derive(Tags)]
#[CombinedApi(Apis)]
pub enum Api {
    MenuApi,
    CRUDApi,
}
