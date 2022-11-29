use yew::{html, Html};
use yew_router::Routable;

use crate::app::components::{login::Login, sys1::Sys1, sys2::Sys2};

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/sys1/:fn_name")]
    Sys1 { fn_name: String },
    #[at("/sys2/:fn_name")]
    Sys2 { fn_name: String },
    #[at("/")]
    Welcome,
}

pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Sys1 { fn_name: _ } => html! {
            <Sys1 />
        },
        AppRoute::Sys2 { fn_name: _ } => html! {
            <Sys2 />
        },
        AppRoute::Welcome => html! {
            <Login />
        },
    }
}
