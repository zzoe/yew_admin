use strum::EnumString;
use yew::{html, Html};
use yew_router::Routable;

use crate::app::components::sys1::{Fn1001, Fn1002};

pub mod login;
pub mod menu;
pub mod sys1;
pub mod sys2;

#[derive(Clone, Routable, PartialEq, Eq, EnumString)]
pub enum FnRoute {
    #[at("/sys1/fn1001")]
    Fn1001,
    #[at("/sys1/fn1002")]
    Fn1002,
}

fn switch_menu(routes: FnRoute) -> Html {
    match routes {
        FnRoute::Fn1001 => html! { <Fn1001 /> },
        FnRoute::Fn1002 => html! { <Fn1002 /> },
    }
}
