use std::str::FromStr;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::components::menu::Menu;
use crate::app::home::fn1::Fn1;
use crate::app::msg::Msg;

pub mod fn1;

#[derive(Clone, Routable, PartialEq)]
pub enum FnRoute {
    #[at("/home/fn1")]
    Fn1,
    #[at("/home/fn2")]
    Fn2,
}

impl FromStr for FnRoute {
    type Err = ();

    fn from_str(fn_name: &str) -> Result<Self, Self::Err> {
        match fn_name.to_lowercase().as_str() {
            "fn1" => Ok(FnRoute::Fn1),
            _ => Ok(FnRoute::Fn2),
        }
    }
}

fn switch_menu(routes: &FnRoute) -> Html {
    match routes {
        FnRoute::Fn1 => html! {
            <Fn1 />
        },
        FnRoute::Fn2 => html! {
            <div class="box">
                <div class="tabs is-toggle">
                    <ul>
                        <li class="is-active"><a>{"Pictures"}</a></li>
                        <li><a>{"Music"}</a></li>
                        <li><a>{"Videos"}</a></li>
                        <li><a>{"Documents"}</a></li>
                    </ul>
                </div>
            </div>
        },
    }
}

pub struct Home;

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        add_scope!(Home, ctx.link());
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        if let Msg::BuggerClick = msg {
            log::info!("received");
        }
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        return html! {
            <div class="columns">
                <div class="column is-narrow">
                    <div class="box">
                        <Menu />
                    </div>
                </div>
                <div class="column">
                    <Switch<FnRoute> render={Switch::render(switch_menu)} />
                </div>
            </div>
        };
    }
}
