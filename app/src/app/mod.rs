use std::cell::RefCell;
use std::rc::Rc;

use yew::scheduler::Shared;
use yew::{classes, html, Component, Context, ContextProvider, Html};
use yew_router::{components::Link, BrowserRouter, Switch};

use msg::Msg;

use crate::app::context::AppContext;
use crate::app::route::{switch, AppRoute};

pub mod components;
pub mod context;
pub mod home;
pub mod msg;
mod route;
pub mod settings;

pub struct App {
    bugger_switch: bool,
    context: Shared<AppContext>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            bugger_switch: false,
            context: Rc::new(RefCell::new(AppContext::default())),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::BuggerClick => {
                self.bugger_switch = !self.bugger_switch;
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let navbar_class = if self.bugger_switch { "is-active" } else { "" };
        let onclick = ctx.link().callback(|_| Msg::BuggerClick);
        let context = Rc::clone(&self.context);

        html! {
            <BrowserRouter>
            <nav class="navbar px-6 py-5" role="navigation" aria-label="main navigation">
              <div class="navbar-brand">
                <Link<AppRoute> classes={"navbar-item"} to={AppRoute::Welcome}>
                  <img src="/resource/rustacean-flat-happy.svg" width="112" height="28" />
                </Link<AppRoute>>

                <a role="button" class={classes!("navbar-burger", navbar_class)} {onclick}
                    aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                </a>
              </div>

              <div id="navbarBasicExample" class={classes!("navbar-menu", navbar_class)}>
                <div class="navbar-start">
                    <Link<AppRoute> classes={"navbar-item"} to={AppRoute::Home { fn_name: "fn1".to_string() }}>
                        { "Home" }
                    </Link<AppRoute>>
                    <Link<AppRoute> classes={"navbar-item"} to={AppRoute::Settings}>
                        {"Settings"}
                    </Link<AppRoute>>

                  <div class="navbar-item has-dropdown is-hoverable">
                    <a class="navbar-link">
                      {"More"}
                    </a>

                    <div class="navbar-dropdown">
                      <a class="navbar-item">
                      {"About"}
                      </a>
                      <a class="navbar-item">
                      {"Jobs"}
                      </a>
                      <a class="navbar-item">
                      {"Contact"}
                      </a>
                      <hr class="navbar-divider" />
                      <a class="navbar-item">
                      {"Report an issue"}
                      </a>
                    </div>
                  </div>
                </div>

                <div class="navbar-end">
                  <div class="navbar-item">
                    <div class="buttons">
                      <a class="button is-primary">
                        <strong>{"Sign up"}</strong>
                      </a>
                      <a class="button is-light">
                      {"Log in"}
                      </a>
                    </div>
                  </div>
                </div>
              </div>
            </nav>

            <main>
                <ContextProvider<Shared<AppContext>> context={context}>
                    <Switch<AppRoute> render={Switch::render(switch)} />
                </ContextProvider<Shared<AppContext>>>
            </main>

            <footer class="footer px-6 py-5">
                <div class="content has-text-centered">
                  <p>{"zoe © www.zoe.zz 鄂ICP备19880211号"}</p>
                </div>
            </footer>
            </BrowserRouter>
        }
    }
}
