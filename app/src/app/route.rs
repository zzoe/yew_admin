use crate::app::home::Home;
use yew::{html, Html};
use yew_router::Routable;

use crate::app::settings::Settings;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/home/:fn_name")]
    Home { fn_name: String },
    #[at("/settings")]
    Settings,
    #[at("/")]
    Welcome,
}

pub fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Home { fn_name: _ } => html! {
            <Home />
        },
        AppRoute::Settings => html! {
            <Settings />
        },
        AppRoute::Welcome => html! {
            <p class="has-text-centered is-size-1">{"Welcome!"}</p>
        },
    }
}
