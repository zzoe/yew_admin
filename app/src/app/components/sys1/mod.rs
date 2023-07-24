use yew::prelude::*;
use yew_router::prelude::*;

pub use fn1001::Fn1001;
pub use fn1002::Fn1002;

use crate::app::components::{menu::Menu, switch_menu, FnRoute};
use crate::app::context::{ContextExt, Module};
use crate::app::msg::Msg;

pub mod fn1001;
pub mod fn1002;

pub struct Sys1;

impl Component for Sys1 {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.insert_scope(Module::Sys1);
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="columns">
                <div class="column is-narrow">
                    <div class="box">
                        <Menu />
                    </div>
                </div>
                <div class="column">
                    <Switch<FnRoute> render={switch_menu} />
                </div>
            </div>
        }
    }
}
