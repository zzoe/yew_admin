use std::collections::HashMap;
use std::convert::AsRef;

use strum::AsRefStr;
use yew::html::Scope;

use crate::app::components::menu::Menu;
use crate::app::home::fn1::Fn1;
use crate::app::home::Home;

#[derive(Clone, AsRefStr)]
pub enum AppScope {
    Home(Scope<Home>),
    Menu(Scope<Menu>),
    Fn1(Scope<Fn1>),
}

impl PartialEq for AppScope {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct AppContext {
    pub scopes: HashMap<String, AppScope>,
}
