use yew::{html, Component, Context, Html};

use crate::app::msg::Msg;

pub struct Login;

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <div class="box">
          <div class="field">
            <label class="label" for="account">{"账号"}</label>
            <div class="control">
              <input id="account" class="input" />
            </div>
          </div>

          <div class="field">
            <label class="label">{"密码"}</label>
            <div class="control">
              <input class="input" type="password" placeholder="********" />
            </div>
          </div>

          <button class="button is-primary">{"登录/注册"}</button>
        </div>
        }
    }
}
