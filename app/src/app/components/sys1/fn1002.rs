use crate::app::context::{ContextExt, Module};
use yew::prelude::*;

use crate::app::msg::Msg;

pub struct Fn1002;

impl Component for Fn1002 {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.insert_scope(Module::Fn1002);
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
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
        }
    }
}
