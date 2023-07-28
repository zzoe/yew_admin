use poem_openapi::Tags;

pub(crate) mod crud;
pub(crate) mod menu;

#[derive(Tags)]
enum ApiTags {
    Crud,
    Menu,
}
